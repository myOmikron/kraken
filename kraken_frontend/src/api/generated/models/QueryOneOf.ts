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
 * @interface QueryOneOf
 */
export interface QueryOneOf {
    /**
     * 
     * @type {SearchType}
     * @memberof QueryOneOf
     */
    email: SearchType;
}

/**
 * Check if a given object implements the QueryOneOf interface.
 */
export function instanceOfQueryOneOf(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "email" in value;

    return isInstance;
}

export function QueryOneOfFromJSON(json: any): QueryOneOf {
    return QueryOneOfFromJSONTyped(json, false);
}

export function QueryOneOfFromJSONTyped(json: any, ignoreDiscriminator: boolean): QueryOneOf {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'email': SearchTypeFromJSON(json['Email']),
    };
}

export function QueryOneOfToJSON(value?: QueryOneOf | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'Email': SearchTypeToJSON(value.email),
    };
}

