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
import {
    FinishRegisterRequestAllOf,
    FinishRegisterRequestAllOfFromJSON,
    FinishRegisterRequestAllOfFromJSONTyped,
    FinishRegisterRequestAllOfToJSON,
} from './';

/**
 * 
 * @export
 * @interface FinishRegisterRequest
 */
export interface FinishRegisterRequest {
    /**
     * 
     * @type {string}
     * @memberof FinishRegisterRequest
     */
    name: string;
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

