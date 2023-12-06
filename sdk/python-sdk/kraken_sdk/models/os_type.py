# coding: utf-8

"""
    kraken

    The core component of kraken-project

    The version of the OpenAPI document: 0.1.0
    Contact: git@omikron.dev
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import json
import pprint
import re  # noqa: F401
from enum import Enum



try:
    from typing import Self
except ImportError:
    from typing_extensions import Self


class OsType(str, Enum):
    """
    A representation of an OS type
    """

    """
    allowed enum values
    """
    UNKNOWN = 'Unknown'
    LINUX = 'Linux'
    WINDOWS = 'Windows'
    APPLE = 'Apple'
    ANDROID = 'Android'
    FREEBSD = 'FreeBSD'

    @classmethod
    def from_json(cls, json_str: str) -> Self:
        """Create an instance of OsType from a JSON string"""
        return cls(json.loads(json_str))

