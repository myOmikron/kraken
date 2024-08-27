/* tslint:disable */
/* eslint-disable */
/**
 * kraken
 * The core component of kraken-project
 *
 * The version of the OpenAPI document: 0.4.2
 * Contact: git@omikron.dev
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


import * as runtime from '../runtime';
import type {
  ApiErrorResponse,
  CreateWorkspaceTagRequest,
  ListWorkspaceTags,
  UpdateWorkspaceTag,
  UuidResponse,
} from '../models';
import {
    ApiErrorResponseFromJSON,
    ApiErrorResponseToJSON,
    CreateWorkspaceTagRequestFromJSON,
    CreateWorkspaceTagRequestToJSON,
    ListWorkspaceTagsFromJSON,
    ListWorkspaceTagsToJSON,
    UpdateWorkspaceTagFromJSON,
    UpdateWorkspaceTagToJSON,
    UuidResponseFromJSON,
    UuidResponseToJSON,
} from '../models';

export interface CreateWorkspaceTagOperationRequest {
    uuid: string;
    createWorkspaceTagRequest: CreateWorkspaceTagRequest;
}

export interface DeleteWorkspaceTagRequest {
    wUuid: string;
    tUuid: string;
}

export interface GetAllWorkspaceTagsRequest {
    uuid: string;
}

export interface UpdateWorkspaceTagRequest {
    wUuid: string;
    tUuid: string;
    updateWorkspaceTag: UpdateWorkspaceTag;
}

/**
 * 
 */
export class WorkspaceTagsApi extends runtime.BaseAPI {

    /**
     * Create a workspace tag.
     */
    async createWorkspaceTagRaw(requestParameters: CreateWorkspaceTagOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<UuidResponse>> {
        if (requestParameters.uuid === null || requestParameters.uuid === undefined) {
            throw new runtime.RequiredError('uuid','Required parameter requestParameters.uuid was null or undefined when calling createWorkspaceTag.');
        }

        if (requestParameters.createWorkspaceTagRequest === null || requestParameters.createWorkspaceTagRequest === undefined) {
            throw new runtime.RequiredError('createWorkspaceTagRequest','Required parameter requestParameters.createWorkspaceTagRequest was null or undefined when calling createWorkspaceTag.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/workspaces/{uuid}/tags`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters.uuid))),
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: CreateWorkspaceTagRequestToJSON(requestParameters.createWorkspaceTagRequest),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => UuidResponseFromJSON(jsonValue));
    }

    /**
     * Create a workspace tag.
     */
    async createWorkspaceTag(requestParameters: CreateWorkspaceTagOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<UuidResponse> {
        const response = await this.createWorkspaceTagRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Requires privileges to access the workspace this tag belongs to.
     * Delete a workspace tag
     */
    async deleteWorkspaceTagRaw(requestParameters: DeleteWorkspaceTagRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters.wUuid === null || requestParameters.wUuid === undefined) {
            throw new runtime.RequiredError('wUuid','Required parameter requestParameters.wUuid was null or undefined when calling deleteWorkspaceTag.');
        }

        if (requestParameters.tUuid === null || requestParameters.tUuid === undefined) {
            throw new runtime.RequiredError('tUuid','Required parameter requestParameters.tUuid was null or undefined when calling deleteWorkspaceTag.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/workspaces/{w_uuid}/tags/{t_uuid}`.replace(`{${"w_uuid"}}`, encodeURIComponent(String(requestParameters.wUuid))).replace(`{${"t_uuid"}}`, encodeURIComponent(String(requestParameters.tUuid))),
            method: 'DELETE',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Requires privileges to access the workspace this tag belongs to.
     * Delete a workspace tag
     */
    async deleteWorkspaceTag(requestParameters: DeleteWorkspaceTagRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.deleteWorkspaceTagRaw(requestParameters, initOverrides);
    }

    /**
     * Retrieve all workspace tags
     */
    async getAllWorkspaceTagsRaw(requestParameters: GetAllWorkspaceTagsRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<ListWorkspaceTags>> {
        if (requestParameters.uuid === null || requestParameters.uuid === undefined) {
            throw new runtime.RequiredError('uuid','Required parameter requestParameters.uuid was null or undefined when calling getAllWorkspaceTags.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/workspaces/{uuid}/tags`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters.uuid))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => ListWorkspaceTagsFromJSON(jsonValue));
    }

    /**
     * Retrieve all workspace tags
     */
    async getAllWorkspaceTags(requestParameters: GetAllWorkspaceTagsRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<ListWorkspaceTags> {
        const response = await this.getAllWorkspaceTagsRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * One of the options must be set  Requires privileges to access the workspace this tags belongs to.
     * Update a workspace tag
     */
    async updateWorkspaceTagRaw(requestParameters: UpdateWorkspaceTagRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters.wUuid === null || requestParameters.wUuid === undefined) {
            throw new runtime.RequiredError('wUuid','Required parameter requestParameters.wUuid was null or undefined when calling updateWorkspaceTag.');
        }

        if (requestParameters.tUuid === null || requestParameters.tUuid === undefined) {
            throw new runtime.RequiredError('tUuid','Required parameter requestParameters.tUuid was null or undefined when calling updateWorkspaceTag.');
        }

        if (requestParameters.updateWorkspaceTag === null || requestParameters.updateWorkspaceTag === undefined) {
            throw new runtime.RequiredError('updateWorkspaceTag','Required parameter requestParameters.updateWorkspaceTag was null or undefined when calling updateWorkspaceTag.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/workspaces/{w_uuid}/tags/{t_uuid}`.replace(`{${"w_uuid"}}`, encodeURIComponent(String(requestParameters.wUuid))).replace(`{${"t_uuid"}}`, encodeURIComponent(String(requestParameters.tUuid))),
            method: 'PUT',
            headers: headerParameters,
            query: queryParameters,
            body: UpdateWorkspaceTagToJSON(requestParameters.updateWorkspaceTag),
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * One of the options must be set  Requires privileges to access the workspace this tags belongs to.
     * Update a workspace tag
     */
    async updateWorkspaceTag(requestParameters: UpdateWorkspaceTagRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.updateWorkspaceTagRaw(requestParameters, initOverrides);
    }

}
