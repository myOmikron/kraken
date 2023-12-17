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
import pprint
import re  # noqa: F401
import json


from typing import Any, ClassVar, Dict, List, Optional
from pydantic import BaseModel, StrictStr
from pydantic import Field
try:
    from typing import Self
except ImportError:
    from typing_extensions import Self

class UpdatePortRequest(BaseModel):
    """
    The request to update a port
    """ # noqa: E501
    comment: Optional[StrictStr] = Field(default=None, description="The comment of the port")
    global_tags: Optional[List[StrictStr]] = Field(default=None, description="Global tags that are linked to the port")
    workspace_tags: Optional[List[StrictStr]] = Field(default=None, description="Workspace tags that are linked to the port")
    __properties: ClassVar[List[str]] = ["comment", "global_tags", "workspace_tags"]

    model_config = {
        "populate_by_name": True,
        "validate_assignment": True
    }


    def to_str(self) -> str:
        """Returns the string representation of the model using alias"""
        return pprint.pformat(self.model_dump(by_alias=True))

    def to_json(self) -> str:
        """Returns the JSON representation of the model using alias"""
        # TODO: pydantic v2: use .model_dump_json(by_alias=True, exclude_unset=True) instead
        return json.dumps(self.to_dict())

    @classmethod
    def from_json(cls, json_str: str) -> Self:
        """Create an instance of UpdatePortRequest from a JSON string"""
        return cls.from_dict(json.loads(json_str))

    def to_dict(self) -> Dict[str, Any]:
        """Return the dictionary representation of the model using alias.

        This has the following differences from calling pydantic's
        `self.model_dump(by_alias=True)`:

        * `None` is only added to the output dict for nullable fields that
          were set at model initialization. Other fields with value `None`
          are ignored.
        """
        _dict = self.model_dump(
            by_alias=True,
            exclude={
            },
            exclude_none=True,
        )
        # set to None if comment (nullable) is None
        # and model_fields_set contains the field
        if self.comment is None and "comment" in self.model_fields_set:
            _dict['comment'] = None

        # set to None if global_tags (nullable) is None
        # and model_fields_set contains the field
        if self.global_tags is None and "global_tags" in self.model_fields_set:
            _dict['global_tags'] = None

        # set to None if workspace_tags (nullable) is None
        # and model_fields_set contains the field
        if self.workspace_tags is None and "workspace_tags" in self.model_fields_set:
            _dict['workspace_tags'] = None

        return _dict

    @classmethod
    def from_dict(cls, obj: Dict) -> Self:
        """Create an instance of UpdatePortRequest from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "comment": obj.get("comment"),
            "global_tags": obj.get("global_tags"),
            "workspace_tags": obj.get("workspace_tags")
        })
        return _obj


