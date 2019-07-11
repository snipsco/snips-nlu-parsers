Snips NLU Parsers
=================

.. image:: https://travis-ci.org/snipsco/snips-nlu-parsers.svg?branch=master
   :target: https://travis-ci.org/snipsco/snips-nlu-parsers

.. image:: https://ci.appveyor.com/api/projects/status/github/snipsco/snips-nlu-parsers?branch=master&svg=true
   :target: https://ci.appveyor.com/project/snipsco/snips-nlu-parsers

This Rust crate provides APIs to extract entities in the context of a Natural Language Understanding (NLU)
task.

A `Python wrapper <python>`_ is also available.

Installation
------------

Add this to your ``Cargo.toml``:

.. code-block:: toml

   [dependencies]
   snips-nlu-parsers = { git = "https://github.com/snipsco/snips-nlu-parsers", tag = "0.1.0" }


Usage
-----

.. code-block:: rust

   use snips_nlu_parsers::{BuiltinEntityKind, BuiltinEntityParserLoader, Language};

   fn parse_entities() {
       let parser = BuiltinEntityParserLoader::new(Language::EN).load().unwrap();
       let entities: Vec<(_, _)> = parser
           .extract_entities("Book me restaurant for two people tomorrow", None)
           .unwrap()
           .into_iter()
           .map(|e| (e.entity_kind, e.range))
           .collect();
       assert_eq!(
           vec![
               (BuiltinEntityKind::Number, 23..26),
               (BuiltinEntityKind::Time, 34..42)
           ],
           entities
       );
   }

License
-------

Licensed under either of
 * Apache License, Version 2.0 (`LICENSE-APACHE <LICENSE-APACHE>`_ or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license (`LICENSE-MIT <LICENSE-MIT>`_) or http://opensource.org/licenses/MIT)
at your option.

Contribution
------------

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
