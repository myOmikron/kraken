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
 * The request to finish the registration of a security key
 * @export
 * @interface FinishRegisterRequest
 */
export interface FinishRegisterRequest {
    /**
     * Name of the key
     * @type {string}
     * @memberof FinishRegisterRequest
     */
    name: string;
}

/**
 * Check if a given object implements the FinishRegisterRequest interface.
 */
export function instanceOfFinishRegisterRequest(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "name" in value;

    return isInstance;
}

export function FinishRegisterRequestFromJSON(json: any): FinishRegisterRequest {
    return FinishRegisterRequestFromJSONTyped(json, false);
}

export function FinishRegisterRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): FinishRegisterRequest {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'name': json['name'],
    };
}

export function FinishRegisterRequestToJSON(value?: FinishRegisterRequest | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'name': value.name,
    };
}

