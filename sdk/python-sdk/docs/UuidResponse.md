# UuidResponse

A common response that contains a single uuid

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uuid** | **str** |  | 

## Example

```python
from kraken_sdk.models.uuid_response import UuidResponse

# TODO update the JSON string below
json = "{}"
# create an instance of UuidResponse from a JSON string
uuid_response_instance = UuidResponse.from_json(json)
# print the JSON string representation of the object
print UuidResponse.to_json()

# convert the object into a dict
uuid_response_dict = uuid_response_instance.to_dict()
# create an instance of UuidResponse from a dict
uuid_response_form_dict = uuid_response.from_dict(uuid_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

