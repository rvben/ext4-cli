"""
ext4-cli: Read ext4 filesystems from image files and block devices.
"""

try:
    from importlib.metadata import version
    __version__ = version("ext4-cli")
except ImportError:
    from importlib_metadata import version
    __version__ = version("ext4-cli")
