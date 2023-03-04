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
import {
    ApiErrorResponse,
    ApiErrorResponseFromJSON,
    ApiErrorResponseToJSON,
    FinishRegisterRequest,
    FinishRegisterRequestFromJSON,
    FinishRegisterRequestToJSON,
    LoginRequest,
    LoginRequestFromJSON,
    LoginRequestToJSON,
} from '../models';

export interface FinishAuthRequest {
    body: object;
}

export interface FinishRegisterOperationRequest {
    finishRegisterRequest: FinishRegisterRequest;
}

export interface LoginOperationRequest {
    loginRequest: LoginRequest;
}

/**
 * 
 */
export class AuthenticationApi extends runtime.BaseAPI {

    /**
     * Finishes the authentication with a security key  Use `startAuth` to retrieve the challenge response data.
     * Finishes the authentication with a security key
     */
    async finishAuthRaw(requestParameters: FinishAuthRequest): Promise<runtime.ApiResponse<void>> {
        if (requestParameters.body === null || requestParameters.body === undefined) {
            throw new runtime.RequiredError('body','Required parameter requestParameters.body was null or undefined when calling finishAuth.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/auth/finishAuth`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: requestParameters.body as any,
        });

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Finishes the authentication with a security key  Use `startAuth` to retrieve the challenge response data.
     * Finishes the authentication with a security key
     */
    async finishAuth(requestParameters: FinishAuthRequest): Promise<void> {
        await this.finishAuthRaw(requestParameters);
    }

    /**
     * Finish the registration of a security key  Use `startRegister` to retrieve the challenge response data.
     * Finish the registration of a security key
     */
    async finishRegisterRaw(requestParameters: FinishRegisterOperationRequest): Promise<runtime.ApiResponse<void>> {
        if (requestParameters.finishRegisterRequest === null || requestParameters.finishRegisterRequest === undefined) {
            throw new runtime.RequiredError('finishRegisterRequest','Required parameter requestParameters.finishRegisterRequest was null or undefined when calling finishRegister.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/auth/finishRegister`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: FinishRegisterRequestToJSON(requestParameters.finishRegisterRequest),
        });

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Finish the registration of a security key  Use `startRegister` to retrieve the challenge response data.
     * Finish the registration of a security key
     */
    async finishRegister(requestParameters: FinishRegisterOperationRequest): Promise<void> {
        await this.finishRegisterRaw(requestParameters);
    }

    /**
     * Login to kraken
     * Login to kraken
     */
    async loginRaw(requestParameters: LoginOperationRequest): Promise<runtime.ApiResponse<void>> {
        if (requestParameters.loginRequest === null || requestParameters.loginRequest === undefined) {
            throw new runtime.RequiredError('loginRequest','Required parameter requestParameters.loginRequest was null or undefined when calling login.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/auth/login`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: LoginRequestToJSON(requestParameters.loginRequest),
        });

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Login to kraken
     * Login to kraken
     */
    async login(requestParameters: LoginOperationRequest): Promise<void> {
        await this.loginRaw(requestParameters);
    }

    /**
     * Log out of this session  Logs a logged-in user out of his session.
     * Log out of this session
     */
    async logoutRaw(): Promise<runtime.ApiResponse<void>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/auth/logout`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Log out of this session  Logs a logged-in user out of his session.
     * Log out of this session
     */
    async logout(): Promise<void> {
        await this.logoutRaw();
    }

    /**
     * Starts the authentication with a security key  Use the `login` endpoint before calling this one.  Proceed with `finishAuth`.
     * Starts the authentication with a security key
     */
    async startAuthRaw(): Promise<runtime.ApiResponse<object>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/auth/startAuth`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse<any>(response);
    }

    /**
     * Starts the authentication with a security key  Use the `login` endpoint before calling this one.  Proceed with `finishAuth`.
     * Starts the authentication with a security key
     */
    async startAuth(): Promise<object> {
        const response = await this.startAuthRaw();
        return await response.value();
    }

    /**
     * Start the registration of a security key  Proceed to the `finishRegister` endpoint.
     * Start the registration of a security key
     */
    async startRegisterRaw(): Promise<runtime.ApiResponse<object>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/auth/startRegister`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse<any>(response);
    }

    /**
     * Start the registration of a security key  Proceed to the `finishRegister` endpoint.
     * Start the registration of a security key
     */
    async startRegister(): Promise<object> {
        const response = await this.startRegisterRaw();
        return await response.value();
    }

    /**
     * Test the current login state  You can use this endpoint to test the current login state of your client.  If logged in, a 200 without a body is returned.
     * Test the current login state
     */
    async testRaw(): Promise<runtime.ApiResponse<void>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/auth/test`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Test the current login state  You can use this endpoint to test the current login state of your client.  If logged in, a 200 without a body is returned.
     * Test the current login state
     */
    async test(): Promise<void> {
        await this.testRaw();
    }

}
