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
  PortResultsPage,
} from '../models';
import {
    ApiErrorResponseFromJSON,
    ApiErrorResponseToJSON,
    PortResultsPageFromJSON,
    PortResultsPageToJSON,
} from '../models';

export interface GetAllPortsRequest {
    uuid: string;
    limit: number;
    offset: number;
    host?: string | null;
}

/**
 * 
 */
export class PortsApi extends runtime.BaseAPI {

    /**
     * List the ports of a workspace
     * List the ports of a workspace
     */
    async getAllPortsRaw(requestParameters: GetAllPortsRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<PortResultsPage>> {
        if (requestParameters.uuid === null || requestParameters.uuid === undefined) {
            throw new runtime.RequiredError('uuid','Required parameter requestParameters.uuid was null or undefined when calling getAllPorts.');
        }

        if (requestParameters.limit === null || requestParameters.limit === undefined) {
            throw new runtime.RequiredError('limit','Required parameter requestParameters.limit was null or undefined when calling getAllPorts.');
        }

        if (requestParameters.offset === null || requestParameters.offset === undefined) {
            throw new runtime.RequiredError('offset','Required parameter requestParameters.offset was null or undefined when calling getAllPorts.');
        }

        const queryParameters: any = {};

        if (requestParameters.limit !== undefined) {
            queryParameters['limit'] = requestParameters.limit;
        }

        if (requestParameters.offset !== undefined) {
            queryParameters['offset'] = requestParameters.offset;
        }

        if (requestParameters.host !== undefined) {
            queryParameters['host'] = requestParameters.host;
        }

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/workspaces/{uuid}/ports`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters.uuid))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => PortResultsPageFromJSON(jsonValue));
    }

    /**
     * List the ports of a workspace
     * List the ports of a workspace
     */
    async getAllPorts(requestParameters: GetAllPortsRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<PortResultsPage> {
        const response = await this.getAllPortsRaw(requestParameters, initOverrides);
        return await response.value();
    }

}