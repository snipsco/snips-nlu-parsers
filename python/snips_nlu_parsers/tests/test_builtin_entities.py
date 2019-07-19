from __future__ import unicode_literals

import unittest
from builtins import str

from snips_nlu_parsers.builtin_entities import (
    get_all_builtin_entities, get_all_gazetteer_entities,
    get_all_grammar_entities, get_all_languages, get_builtin_entity_examples,
    get_builtin_entity_shortname, get_supported_entities,
    get_supported_gazetteer_entities, get_supported_grammar_entities,
    get_complete_entity_ontology, get_language_entity_ontology)


class TestBuiltinEntities(unittest.TestCase):
    def test_should_get_all_languages(self):
        # When
        all_languages = get_all_languages()

        # Then
        self.assertIn("en", all_languages)
        self.assertIn("fr", all_languages)
        for language in all_languages:
            self.assertIsInstance(language, str)

    def test_should_get_builtin_entity_shortname(self):
        # Given
        entity_name = "snips/amountOfMoney"

        # When
        short_name = get_builtin_entity_shortname(entity_name)

        # Then
        self.assertEqual("AmountOfMoney", short_name)

    def test_should_get_all_builtin_entities(self):
        # When
        all_builtins = get_all_builtin_entities()

        # Then
        self.assertIn("snips/number", all_builtins)
        self.assertIn("snips/musicArtist", all_builtins)
        for builtin in all_builtins:
            self.assertIsInstance(builtin, str)

    def test_should_get_all_grammar_entities(self):
        # When
        all_grammar_entities = get_all_grammar_entities()

        # Then
        self.assertIn("snips/number", all_grammar_entities)
        self.assertNotIn("snips/musicArtist", all_grammar_entities)
        for builtin in all_grammar_entities:
            self.assertIsInstance(builtin, str)

    def test_should_get_all_gazetteer_entities(self):
        # When
        all_gazetteer_entities = get_all_gazetteer_entities()

        # Then
        self.assertNotIn("snips/number", all_gazetteer_entities)
        self.assertIn("snips/musicArtist", all_gazetteer_entities)
        for builtin in all_gazetteer_entities:
            self.assertIsInstance(builtin, str)

    def test_should_get_supported_builtin_entities(self):
        # When
        supported_entities = get_supported_entities("en")

        # Then
        self.assertIn("snips/number", supported_entities)
        self.assertIn("snips/datetime", supported_entities)
        for builtin in supported_entities:
            self.assertIsInstance(builtin, str)

    def test_should_get_supported_gazetteer_entities(self):
        # When
        supported_entities = get_supported_gazetteer_entities("fr")

        # Then
        self.assertIn("snips/musicArtist", supported_entities)
        self.assertIn("snips/musicAlbum", supported_entities)
        self.assertNotIn("snips/number", supported_entities)
        for builtin in supported_entities:
            self.assertIsInstance(builtin, str)

    def test_should_get_supported_grammar_entities(self):
        # When
        supported_entities = get_supported_grammar_entities("en")

        # Then
        self.assertIn("snips/number", supported_entities)
        self.assertIn("snips/datetime", supported_entities)
        for builtin in supported_entities:
            self.assertIsInstance(builtin, str)

    def test_should_get_builtin_entity_examples(self):
        for language in get_all_languages():
            for builtin_entity in get_supported_entities(language):
                examples = get_builtin_entity_examples(builtin_entity,
                                                       language)
                self.assertGreaterEqual(len(examples), 1)

    def test_should_get_complete_entity_ontology(self):
        # When
        complete_ontology = get_complete_entity_ontology()

        # Then
        self.assertTrue(len(complete_ontology) > 0)
        for configuration in complete_ontology:
            self.assertTrue("language" in configuration)
            self.assertTrue("entities" in configuration)

    def test_should_get_language_entity_ontology(self):
        # When
        language_entity_ontology = get_language_entity_ontology("en")

        # Then
        self.assertEqual(language_entity_ontology["language"], "en")
        self.assertTrue(len(language_entity_ontology["entities"]) > 0)
