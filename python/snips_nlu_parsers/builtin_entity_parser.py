import json
from _ctypes import byref
from builtins import bytes, str
from ctypes import c_char_p, c_int, c_void_p, string_at
from pathlib import Path

from snips_nlu_parsers.utils import (
    CStringArray, check_ffi_error, lib, string_pointer)


class BuiltinEntityParser(object):
    def __init__(self, parser):
        self._parser = parser

    @classmethod
    def build(cls, language, gazetteer_entity_parser_path=None):
        """Builds a `BuiltinEntityParser`

        Args:
            language (str): Language identifier
            gazetteer_entity_parser_path (str, optional): Path to a gazetteer
                entity parser. If None, the builtin entity parser will only
                use grammar entities.
        """
        if isinstance(gazetteer_entity_parser_path, Path):
            gazetteer_entity_parser_path = str(gazetteer_entity_parser_path)
        if not isinstance(language, str):
            raise TypeError("Expected language to be of type 'str' but found:"
                            " %s" % type(language))
        parser_config = dict(
            language=language.upper(),
            gazetteer_parser_path=gazetteer_entity_parser_path)
        parser = c_void_p()
        json_parser_config = bytes(json.dumps(parser_config), encoding="utf8")
        exit_code = lib.snips_nlu_parsers_create_builtin_entity_parser(
            byref(parser), json_parser_config)
        check_ffi_error(exit_code, "Something went wrong while creating the "
                                   "builtin entity parser")
        return cls(parser)

    def parse(self, text, scope=None, max_alternative_resolved_values=5):
        """Extracts builtin entities from *text*

        Args:
            text (str): Input
            scope (list of str, optional): List of builtin entity labels. If
                defined, the parser will extract entities using the provided
                scope instead of the entire scope of all available entities.
                This allows to look for specifics builtin entity kinds.
            max_alternative_resolved_values (int, optional): Maximum number of
                alternative resolved values to return in addition to the top
                one (default 5).

        Returns:
            list of dict: The list of extracted entities
        """
        if not isinstance(text, str):
            raise TypeError("Expected language to be of type 'str' but found: "
                            "%s" % type(text))
        if scope is not None:
            if not all(isinstance(e, str) for e in scope):
                raise TypeError(
                    "Expected scope to contain objects of type 'str'")
            scope = [e.encode("utf8") for e in scope]
            arr = CStringArray()
            arr.size = c_int(len(scope))
            arr.data = (c_char_p * len(scope))(*scope)
            scope = byref(arr)

        with string_pointer(c_char_p()) as ptr:
            exit_code = lib.snips_nlu_parsers_extract_builtin_entities_json(
                self._parser, text.encode("utf8"), scope,
                max_alternative_resolved_values, byref(ptr))
            check_ffi_error(exit_code, "Something went wrong when extracting "
                                       "builtin entities")
            result = string_at(ptr)
            return json.loads(result.decode("utf8"))

    def extend_gazetteer_entity(self, entity_name, entity_values):
        """Extends a builtin gazetteer entity with custom values

        Args:
            entity_name (str): Gazetteer entity identifier
            entity_values (list of dict): List of entity values represented as
                dictionaries with a 'raw_value' key and a 'resolved_value' key

        Returns:
            The same object, updated.

        Raises:
            ValueError: when the entity name is unknown or not present in the
                parser
        """
        if not entity_values:
            return self
        entity_values_json = bytes(json.dumps(entity_values), encoding="utf8")
        exit_code = lib.snips_nlu_parsers_extend_gazetteer_entity_json(
            self._parser, entity_name.encode("utf8"), entity_values_json)
        check_ffi_error(exit_code, "Something went wrong when extending the "
                                   "builtin entity '%s'" % entity_name)
        return self

    def persist(self, path):
        """Persists the builtin entity parser on disk at the provided path"""
        if isinstance(path, Path):
            path = str(path)
        exit_code = lib.snips_nlu_parsers_persist_builtin_entity_parser(
            self._parser, path.encode("utf8"))
        check_ffi_error(exit_code, "Something went wrong when persisting the "
                                   "builtin entity parser")

    @classmethod
    def from_path(cls, parser_path):
        """Creates a :class:`BuiltinEntityParser` from a builtin entity parser
        persisted on disk
        """
        if isinstance(parser_path, Path):
            parser_path = str(parser_path)
        parser = c_void_p()
        parser_path = bytes(parser_path, encoding="utf8")
        exit_code = lib.snips_nlu_parsers_load_builtin_entity_parser(
            byref(parser), parser_path)
        check_ffi_error(exit_code, "Something went wrong when loading the "
                                   "builtin entity parser")
        return cls(parser)

    def __del__(self):
        if lib is not None and self._parser is not None:
            lib.snips_nlu_parsers_destroy_builtin_entity_parser(self._parser)
