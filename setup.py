import os
import sys
import re
from setuptools import setup
from setuptools.command.sdist import sdist as SdistCommand

with open('polaroid/__init__.py') as f:
    version = re.search(r'^__version__\s*=\s*[\'"]([^\'"]*)[\'"]', f.read(), re.MULTILINE).group(1)

try:
    from setuptools_rust import Binding, RustExtension
except ImportError:
    import subprocess
    errno = subprocess.call(
        [sys.executable, '-m', 'pip', 'install', 'setuptools-rust'])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import Binding, RustExtension

class CargoModifiedSdist(SdistCommand):
    """Modifies Cargo.toml to use an absolute rather than a relative path
    The current implementation of PEP 517 in pip always does builds in an
    isolated temporary directory. This causes problems with the build, because
    Cargo.toml necessarily refers to the current version of pyo3 by a relative
    path.
    Since these sdists are never meant to be used for anything other than
    tox / pip installs, at sdist build time, we will modify the Cargo.toml
    in the sdist archive to include an *absolute* path to pyo3.
    """

    def make_release_tree(self, base_dir, files):
        """Stages the files to be included in archives"""
        super().make_release_tree(base_dir, files)

        import toml

        # Cargo.toml is now staged and ready to be modified
        cargo_loc = os.path.join(base_dir, "Cargo.toml")
        assert os.path.exists(cargo_loc)

        with open(cargo_loc, "r") as f:
            cargo_toml = toml.load(f)

        rel_pyo3_path = cargo_toml["dependencies"]["pyo3"]["path"]
        base_path = os.path.dirname(__file__)
        abs_pyo3_path = os.path.abspath(os.path.join(base_path, rel_pyo3_path))

        cargo_toml["dependencies"]["pyo3"]["path"] = abs_pyo3_path

        with open(cargo_loc, "w") as f:
            toml.dump(cargo_toml, f)

setup_requires = ['setuptools-rust>=0.9.2']
install_requires = []

setup(name='polaroid',
      version=version,
      description="Hyper Fast and safe image manipulation library for python . Powered by rust.",
      long_description=open("README.md").read(),
      long_description_content_type="text/markdown",
      license="MIT",
      url="https://github.com/Daggy1234/polaroid",
      project_urls={
          "Issue tracker": "https://github.com/Daggy1234/polaroid/issues",
          "discord": "https://server.daggy.tech"
      },
      classifiers=[
          "Intended Audience :: Developers",
          'License :: OSI Approved :: MIT License',
          'Development Status :: 3 - Alpha',
          'Intended Audience :: Developers',
          'Programming Language :: Python',
          'Programming Language :: Rust',
          "Operating System :: OS Independent",
          'Natural Language :: English',
          "Programming Language :: Python :: 3.6",
          "Programming Language :: Python :: 3.7",
          "Programming Language :: Python :: 3.8",
          'Topic :: Internet',
          'Topic :: Software Development :: Libraries',
          'Topic :: Software Development :: Libraries :: Python Modules',
          'Topic :: Utilities',
          'Topic :: Images'
      ],
      rust_extensions=[
          RustExtension('polaroid.polaroid', 'Cargo.toml', binding=Binding.PyO3)],
      setup_requires=setup_requires,
      include_package_data=True,
      packages=['polaroid'],
      zip_safe=False,
      cmdclass={"sdist": CargoModifiedSdist})