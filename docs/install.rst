Installation
============

.. highlight:: console

Precompiled Wheels
------------------

``Polaroid`` Python module is implemented in Rust, but the Rust compiler
is only required if your platform does not have precompiled wheels available.
Currently, we provide `wheels <https://pythonwheels.com/>`_ for the following
platforms and implementations:

* **Linux x86-64**: CPython 3.6, 3.7, 3.8, 3.9
* **OSX x86-64**: CPython 3.6, 3.7, 3.8, 3.9
* **Windows x86-64**: CPython 3.6, 3.7, 3.8
* **Linux i686**: CPython 3.6, 3.7, 3.8, 3.9
* **Linux aarch64**: CPython 3.6, 3.7, 3.8, 3.9
* **OSX arm**: CPython 3.9

For addition platforms Please open an issue or join our discord server!

Downloading and installing from a wheel is then as simple as::

  $ pip install polaroid


Building from source
--------------------

In order to build the code from source, you will need to have
the Rust compiler installed and available in your ``$PATH``. See
`documentation on rust-lang.org <https://forge.rust-lang.org/other-installation-methods.html>`_
to learn how to install Rust on your machine.

Then installing with ``pip`` will build the pacakge::

  $ pip install polaroid


MUSL Linux wheels
-----------------

Sometimes you might use a docker distro like alpine linux that uses musl. These wheels aren't available from pypi and will need to built from source! 
