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
  SettingsFull,
  UpdateSettingsRequest,
} from '../models';
import {
    ApiErrorResponseFromJSON,
    ApiErrorResponseToJSON,
    SettingsFullFromJSON,
    SettingsFullToJSON,
    UpdateSettingsRequestFromJSON,
    UpdateSettingsRequestToJSON,
} from '../models';

export interface UpdateSettingsOperationRequest {
    updateSettingsRequest: UpdateSettingsRequest;
}

/**
 * 
 */
export class SettingsManagementApi extends runtime.BaseAPI {

    /**
     * Retrieve the currently active settings
     */
    async getSettingsRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<SettingsFull>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/admin/settings`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => SettingsFullFromJSON(jsonValue));
    }

    /**
     * Retrieve the currently active settings
     */
    async getSettings(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<SettingsFull> {
        const response = await this.getSettingsRaw(initOverrides);
        return await response.value();
    }

    /**
     * Update the settings
     */
    async updateSettingsRaw(requestParameters: UpdateSettingsOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters.updateSettingsRequest === null || requestParameters.updateSettingsRequest === undefined) {
            throw new runtime.RequiredError('updateSettingsRequest','Required parameter requestParameters.updateSettingsRequest was null or undefined when calling updateSettings.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/admin/settings`,
            method: 'PUT',
            headers: headerParameters,
            query: queryParameters,
            body: UpdateSettingsRequestToJSON(requestParameters.updateSettingsRequest),
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Update the settings
     */
    async updateSettings(requestParameters: UpdateSettingsOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.updateSettingsRaw(requestParameters, initOverrides);
    }

}
