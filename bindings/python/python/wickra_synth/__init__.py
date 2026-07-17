"""Wickra Synth — deterministic synthetic market microstructure.

Build a :class:`Synth` from a spec JSON, drive it with command JSONs, and
read back the generated data. The same command protocol crosses every language
binding, so this Python front-end drives the exact same core as the native CLI.
"""

from ._wickra_synth import Synth, __version__

__all__ = ["Synth", "__version__"]
