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

import { exists, mapValues } from '../runtime';
/**
 * 
 * @export
 * @interface CreateUserRequest
 */
export interface CreateUserRequest {
    /**
     * 
     * @type {string}
     * @memberof CreateUserRequest
     */
    username: string;
    /**
     * 
     * @type {string}
     * @memberof CreateUserRequest
     */
    displayName: string;
    /**
     * 
     * @type {string}
     * @memberof CreateUserRequest
     */
    password: string;
    /**
     * 
     * @type {boolean}
     * @memberof CreateUserRequest
     */
    admin: boolean;
}

export function CreateUserRequestFromJSON(json: any): CreateUserRequest {
    return CreateUserRequestFromJSONTyped(json, false);
}

export function CreateUserRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): CreateUserRequest {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'username': json['username'],
        'displayName': json['display_name'],
        'password': json['password'],
        'admin': json['admin'],
    };
}

export function CreateUserRequestToJSON(value?: CreateUserRequest | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'username': value.username,
        'display_name': value.displayName,
        'password': value.password,
        'admin': value.admin,
    };
}


