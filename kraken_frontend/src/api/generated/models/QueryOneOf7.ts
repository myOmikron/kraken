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

import { exists, mapValues } from '../runtime';
import type { SearchType } from './SearchType';
import {
    SearchTypeFromJSON,
    SearchTypeFromJSONTyped,
    SearchTypeToJSON,
} from './SearchType';

/**
 * 
 * @export
 * @interface QueryOneOf7
 */
export interface QueryOneOf7 {
    /**
     * 
     * @type {SearchType}
     * @memberof QueryOneOf7
     */
    vin: SearchType;
}

/**
 * Check if a given object implements the QueryOneOf7 interface.
 */
export function instanceOfQueryOneOf7(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "vin" in value;

    return isInstance;
}

export function QueryOneOf7FromJSON(json: any): QueryOneOf7 {
    return QueryOneOf7FromJSONTyped(json, false);
}

export function QueryOneOf7FromJSONTyped(json: any, ignoreDiscriminator: boolean): QueryOneOf7 {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'vin': SearchTypeFromJSON(json['Vin']),
    };
}

export function QueryOneOf7ToJSON(value?: QueryOneOf7 | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'Vin': SearchTypeToJSON(value.vin),
    };
}

