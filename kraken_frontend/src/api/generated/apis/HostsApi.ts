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
  CreateHostRequest,
  FullAggregationSource,
  FullHost,
  GetAllHostsQuery,
  HostRelations,
  HostResultsPage,
  ListFindings,
  UpdateHostRequest,
  UuidsResponse,
} from '../models';
import {
    ApiErrorResponseFromJSON,
    ApiErrorResponseToJSON,
    CreateHostRequestFromJSON,
    CreateHostRequestToJSON,
    FullAggregationSourceFromJSON,
    FullAggregationSourceToJSON,
    FullHostFromJSON,
    FullHostToJSON,
    GetAllHostsQueryFromJSON,
    GetAllHostsQueryToJSON,
    HostRelationsFromJSON,
    HostRelationsToJSON,
    HostResultsPageFromJSON,
    HostResultsPageToJSON,
    ListFindingsFromJSON,
    ListFindingsToJSON,
    UpdateHostRequestFromJSON,
    UpdateHostRequestToJSON,
    UuidsResponseFromJSON,
    UuidsResponseToJSON,
} from '../models';

export interface CreateHostOperationRequest {
    uuid: string;
    createHostRequest: CreateHostRequest;
}

export interface DeleteHostRequest {
    wUuid: string;
    hUuid: string;
}

export interface GetAllHostsRequest {
    uuid: string;
    getAllHostsQuery: GetAllHostsQuery;
}

export interface GetHostRequest {
    wUuid: string;
    hUuid: string;
}

export interface GetHostFindingsRequest {
    wUuid: string;
    hUuid: string;
}

export interface GetHostRelationsRequest {
    wUuid: string;
    hUuid: string;
}

export interface GetHostSourcesRequest {
    wUuid: string;
    hUuid: string;
}

export interface UpdateHostOperationRequest {
    wUuid: string;
    hUuid: string;
    updateHostRequest: UpdateHostRequest;
}

/**
 * 
 */
export class HostsApi extends runtime.BaseAPI {

    /**
     * This endpoint also accepts networks inserting all their ips as hosts
     * Manually add a host
     */
    async createHostRaw(requestParameters: CreateHostOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<UuidsResponse>> {
        if (requestParameters.uuid === null || requestParameters.uuid === undefined) {
            throw new runtime.RequiredError('uuid','Required parameter requestParameters.uuid was null or undefined when calling createHost.');
        }

        if (requestParameters.createHostRequest === null || requestParameters.createHostRequest === undefined) {
            throw new runtime.RequiredError('createHostRequest','Required parameter requestParameters.createHostRequest was null or undefined when calling createHost.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/workspaces/{uuid}/hosts`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters.uuid))),
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: CreateHostRequestToJSON(requestParameters.createHostRequest),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => UuidsResponseFromJSON(jsonValue));
    }

    /**
     * This endpoint also accepts networks inserting all their ips as hosts
     * Manually add a host
     */
    async createHost(requestParameters: CreateHostOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<UuidsResponse> {
        const response = await this.createHostRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * This only deletes the aggregation. The raw results are still in place
     * Delete the host
     */
    async deleteHostRaw(requestParameters: DeleteHostRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters.wUuid === null || requestParameters.wUuid === undefined) {
            throw new runtime.RequiredError('wUuid','Required parameter requestParameters.wUuid was null or undefined when calling deleteHost.');
        }

        if (requestParameters.hUuid === null || requestParameters.hUuid === undefined) {
            throw new runtime.RequiredError('hUuid','Required parameter requestParameters.hUuid was null or undefined when calling deleteHost.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/workspaces/{w_uuid}/hosts/{h_uuid}`.replace(`{${"w_uuid"}}`, encodeURIComponent(String(requestParameters.wUuid))).replace(`{${"h_uuid"}}`, encodeURIComponent(String(requestParameters.hUuid))),
            method: 'DELETE',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * This only deletes the aggregation. The raw results are still in place
     * Delete the host
     */
    async deleteHost(requestParameters: DeleteHostRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.deleteHostRaw(requestParameters, initOverrides);
    }

    /**
     * Hosts are created out of aggregating data or by user input. They represent a single host and can be created by providing an IP address
     * Retrieve all hosts.
     */
    async getAllHostsRaw(requestParameters: GetAllHostsRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<HostResultsPage>> {
        if (requestParameters.uuid === null || requestParameters.uuid === undefined) {
            throw new runtime.RequiredError('uuid','Required parameter requestParameters.uuid was null or undefined when calling getAllHosts.');
        }

        if (requestParameters.getAllHostsQuery === null || requestParameters.getAllHostsQuery === undefined) {
            throw new runtime.RequiredError('getAllHostsQuery','Required parameter requestParameters.getAllHostsQuery was null or undefined when calling getAllHosts.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/workspaces/{uuid}/hosts/all`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters.uuid))),
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: GetAllHostsQueryToJSON(requestParameters.getAllHostsQuery),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => HostResultsPageFromJSON(jsonValue));
    }

    /**
     * Hosts are created out of aggregating data or by user input. They represent a single host and can be created by providing an IP address
     * Retrieve all hosts.
     */
    async getAllHosts(requestParameters: GetAllHostsRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<HostResultsPage> {
        const response = await this.getAllHostsRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Retrieve all information about a single host
     */
    async getHostRaw(requestParameters: GetHostRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<FullHost>> {
        if (requestParameters.wUuid === null || requestParameters.wUuid === undefined) {
            throw new runtime.RequiredError('wUuid','Required parameter requestParameters.wUuid was null or undefined when calling getHost.');
        }

        if (requestParameters.hUuid === null || requestParameters.hUuid === undefined) {
            throw new runtime.RequiredError('hUuid','Required parameter requestParameters.hUuid was null or undefined when calling getHost.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/workspaces/{w_uuid}/hosts/{h_uuid}`.replace(`{${"w_uuid"}}`, encodeURIComponent(String(requestParameters.wUuid))).replace(`{${"h_uuid"}}`, encodeURIComponent(String(requestParameters.hUuid))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => FullHostFromJSON(jsonValue));
    }

    /**
     * Retrieve all information about a single host
     */
    async getHost(requestParameters: GetHostRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<FullHost> {
        const response = await this.getHostRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Get a host\'s findings
     */
    async getHostFindingsRaw(requestParameters: GetHostFindingsRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<ListFindings>> {
        if (requestParameters.wUuid === null || requestParameters.wUuid === undefined) {
            throw new runtime.RequiredError('wUuid','Required parameter requestParameters.wUuid was null or undefined when calling getHostFindings.');
        }

        if (requestParameters.hUuid === null || requestParameters.hUuid === undefined) {
            throw new runtime.RequiredError('hUuid','Required parameter requestParameters.hUuid was null or undefined when calling getHostFindings.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/workspaces/{w_uuid}/hosts/{h_uuid}/findings`.replace(`{${"w_uuid"}}`, encodeURIComponent(String(requestParameters.wUuid))).replace(`{${"h_uuid"}}`, encodeURIComponent(String(requestParameters.hUuid))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => ListFindingsFromJSON(jsonValue));
    }

    /**
     * Get a host\'s findings
     */
    async getHostFindings(requestParameters: GetHostFindingsRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<ListFindings> {
        const response = await this.getHostFindingsRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Get a host\'s direct relations
     */
    async getHostRelationsRaw(requestParameters: GetHostRelationsRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<HostRelations>> {
        if (requestParameters.wUuid === null || requestParameters.wUuid === undefined) {
            throw new runtime.RequiredError('wUuid','Required parameter requestParameters.wUuid was null or undefined when calling getHostRelations.');
        }

        if (requestParameters.hUuid === null || requestParameters.hUuid === undefined) {
            throw new runtime.RequiredError('hUuid','Required parameter requestParameters.hUuid was null or undefined when calling getHostRelations.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/workspaces/{w_uuid}/hosts/{h_uuid}/relations`.replace(`{${"w_uuid"}}`, encodeURIComponent(String(requestParameters.wUuid))).replace(`{${"h_uuid"}}`, encodeURIComponent(String(requestParameters.hUuid))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => HostRelationsFromJSON(jsonValue));
    }

    /**
     * Get a host\'s direct relations
     */
    async getHostRelations(requestParameters: GetHostRelationsRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<HostRelations> {
        const response = await this.getHostRelationsRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Get all data sources which referenced this host
     */
    async getHostSourcesRaw(requestParameters: GetHostSourcesRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<FullAggregationSource>> {
        if (requestParameters.wUuid === null || requestParameters.wUuid === undefined) {
            throw new runtime.RequiredError('wUuid','Required parameter requestParameters.wUuid was null or undefined when calling getHostSources.');
        }

        if (requestParameters.hUuid === null || requestParameters.hUuid === undefined) {
            throw new runtime.RequiredError('hUuid','Required parameter requestParameters.hUuid was null or undefined when calling getHostSources.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/workspaces/{w_uuid}/hosts/{h_uuid}/sources`.replace(`{${"w_uuid"}}`, encodeURIComponent(String(requestParameters.wUuid))).replace(`{${"h_uuid"}}`, encodeURIComponent(String(requestParameters.hUuid))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => FullAggregationSourceFromJSON(jsonValue));
    }

    /**
     * Get all data sources which referenced this host
     */
    async getHostSources(requestParameters: GetHostSourcesRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<FullAggregationSource> {
        const response = await this.getHostSourcesRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * You must include at least on parameter
     * Update a host
     */
    async updateHostRaw(requestParameters: UpdateHostOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters.wUuid === null || requestParameters.wUuid === undefined) {
            throw new runtime.RequiredError('wUuid','Required parameter requestParameters.wUuid was null or undefined when calling updateHost.');
        }

        if (requestParameters.hUuid === null || requestParameters.hUuid === undefined) {
            throw new runtime.RequiredError('hUuid','Required parameter requestParameters.hUuid was null or undefined when calling updateHost.');
        }

        if (requestParameters.updateHostRequest === null || requestParameters.updateHostRequest === undefined) {
            throw new runtime.RequiredError('updateHostRequest','Required parameter requestParameters.updateHostRequest was null or undefined when calling updateHost.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/workspaces/{w_uuid}/hosts/{h_uuid}`.replace(`{${"w_uuid"}}`, encodeURIComponent(String(requestParameters.wUuid))).replace(`{${"h_uuid"}}`, encodeURIComponent(String(requestParameters.hUuid))),
            method: 'PUT',
            headers: headerParameters,
            query: queryParameters,
            body: UpdateHostRequestToJSON(requestParameters.updateHostRequest),
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * You must include at least on parameter
     * Update a host
     */
    async updateHost(requestParameters: UpdateHostOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.updateHostRaw(requestParameters, initOverrides);
    }

}
