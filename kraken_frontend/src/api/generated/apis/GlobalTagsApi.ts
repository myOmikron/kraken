/* tslint:disable */
/* eslint-disable */
/**
 * kraken
 * The core component of kraken-project
 *
 * The version of the OpenAPI document: 0.1.0
 * Contact: git@omikron.dev
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


import * as runtime from '../runtime';
import type {
  ApiErrorResponse,
  CreateGlobalTagRequest,
  ListGlobalTags,
  UpdateGlobalTag,
  UuidResponse,
} from '../models';
import {
    ApiErrorResponseFromJSON,
    ApiErrorResponseToJSON,
    CreateGlobalTagRequestFromJSON,
    CreateGlobalTagRequestToJSON,
    ListGlobalTagsFromJSON,
    ListGlobalTagsToJSON,
    UpdateGlobalTagFromJSON,
    UpdateGlobalTagToJSON,
    UuidResponseFromJSON,
    UuidResponseToJSON,
} from '../models';

export interface CreateGlobalTagOperationRequest {
    createGlobalTagRequest: CreateGlobalTagRequest;
}

export interface DeleteGlobalTagRequest {
    uuid: string;
}

export interface UpdateGlobalTagRequest {
    uuid: string;
    updateGlobalTag: UpdateGlobalTag;
}

/**
 * 
 */
export class GlobalTagsApi extends runtime.BaseAPI {

    /**
     * Create a global tag.  This action requires admin privileges.
     * Create a global tag.
     */
    async createGlobalTagRaw(requestParameters: CreateGlobalTagOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<UuidResponse>> {
        if (requestParameters.createGlobalTagRequest === null || requestParameters.createGlobalTagRequest === undefined) {
            throw new runtime.RequiredError('createGlobalTagRequest','Required parameter requestParameters.createGlobalTagRequest was null or undefined when calling createGlobalTag.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/admin/globalTags`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: CreateGlobalTagRequestToJSON(requestParameters.createGlobalTagRequest),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => UuidResponseFromJSON(jsonValue));
    }

    /**
     * Create a global tag.  This action requires admin privileges.
     * Create a global tag.
     */
    async createGlobalTag(requestParameters: CreateGlobalTagOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<UuidResponse> {
        const response = await this.createGlobalTagRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Delete a global tag  Requires admin privileges.
     * Delete a global tag
     */
    async deleteGlobalTagRaw(requestParameters: DeleteGlobalTagRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters.uuid === null || requestParameters.uuid === undefined) {
            throw new runtime.RequiredError('uuid','Required parameter requestParameters.uuid was null or undefined when calling deleteGlobalTag.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/admin/globalTags/{uuid}`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters.uuid))),
            method: 'DELETE',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Delete a global tag  Requires admin privileges.
     * Delete a global tag
     */
    async deleteGlobalTag(requestParameters: DeleteGlobalTagRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.deleteGlobalTagRaw(requestParameters, initOverrides);
    }

    /**
     * Retrieve all global tags
     * Retrieve all global tags
     */
    async getAllGlobalTagsRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<ListGlobalTags>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/globalTags`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => ListGlobalTagsFromJSON(jsonValue));
    }

    /**
     * Retrieve all global tags
     * Retrieve all global tags
     */
    async getAllGlobalTags(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<ListGlobalTags> {
        const response = await this.getAllGlobalTagsRaw(initOverrides);
        return await response.value();
    }

    /**
     * Update a global tag  One of the options must be set  Requires admin privileges.
     * Update a global tag
     */
    async updateGlobalTagRaw(requestParameters: UpdateGlobalTagRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters.uuid === null || requestParameters.uuid === undefined) {
            throw new runtime.RequiredError('uuid','Required parameter requestParameters.uuid was null or undefined when calling updateGlobalTag.');
        }

        if (requestParameters.updateGlobalTag === null || requestParameters.updateGlobalTag === undefined) {
            throw new runtime.RequiredError('updateGlobalTag','Required parameter requestParameters.updateGlobalTag was null or undefined when calling updateGlobalTag.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/admin/globalTags/{uuid}`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters.uuid))),
            method: 'PUT',
            headers: headerParameters,
            query: queryParameters,
            body: UpdateGlobalTagToJSON(requestParameters.updateGlobalTag),
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Update a global tag  One of the options must be set  Requires admin privileges.
     * Update a global tag
     */
    async updateGlobalTag(requestParameters: UpdateGlobalTagRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.updateGlobalTagRaw(requestParameters, initOverrides);
    }

}
