Snips NLU Parsers
=================

Installation
------------

--------------------------------------
Linux x86 / MacOS (>= 10.11) / Windows
--------------------------------------

Wheels and source distribution are available for Python2.7 as well as Python >= 3.4

.. code-block:: bash

    pip install snips-nlu-parsers

---------------
Other platforms
---------------

This package can be installed via pip from a source distribution. As it contains
some ``rust`` code, ``rust`` must be installed on your machine.

To install Rust, run the following in your terminal, then follow the onscreen instructions:

.. code-block:: bash

    curl https://sh.rustup.rs -sSf | sh


You will also need the python lib ``setuptools_rust``:

.. code-block:: bash

    pip install setuptools_rust

Finally, you can install ``snips-nlu-parsers`` using pip:

.. code-block:: bash

    pip install snips-nlu-parsers


Usage
-----

.. code-block:: python

   >>> from snips_nlu_parsers import BuiltinEntityParser
   >>> import json
   >>> parser = BuiltinEntityParser.build(language="en")
   >>> parsing = parser.parse("what will be the weather in three days ?")
   >>> print(json.dumps(parsing, indent=2))
   [
     {
       "value": "in three days",
       "range": {
         "start": 25,
         "end": 38
       },
       "entity": {
         "kind": "InstantTime",
         "value": "2019-02-24 00:00:00 +01:00",
         "grain": "Day",
         "precision": "Exact"
       },
       "entity_kind": "snips/datetime"
     }
   ]
