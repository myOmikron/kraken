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
 * A full representation of a bearer token
 * @export
 * @interface FullBearerToken
 */
export interface FullBearerToken {
    /**
     * The primary key of the token
     * @type {string}
     * @memberof FullBearerToken
     */
    uuid: string;
    /**
     * The name that is used for identification
     * @type {string}
     * @memberof FullBearerToken
     */
    name: string;
    /**
     * The token that is used in the authorization header
     * @type {string}
     * @memberof FullBearerToken
     */
    token: string;
}

/**
 * Check if a given object implements the FullBearerToken interface.
 */
export function instanceOfFullBearerToken(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "uuid" in value;
    isInstance = isInstance && "name" in value;
    isInstance = isInstance && "token" in value;

    return isInstance;
}

export function FullBearerTokenFromJSON(json: any): FullBearerToken {
    return FullBearerTokenFromJSONTyped(json, false);
}

export function FullBearerTokenFromJSONTyped(json: any, ignoreDiscriminator: boolean): FullBearerToken {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'uuid': json['uuid'],
        'name': json['name'],
        'token': json['token'],
    };
}

export function FullBearerTokenToJSON(value?: FullBearerToken | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'uuid': value.uuid,
        'name': value.name,
        'token': value.token,
    };
}

