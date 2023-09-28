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
 * @interface SearchTypeOneOf2
 */
export interface SearchTypeOneOf2 {
    /**
     * A regex search pattern
     * @type {string}
     * @memberof SearchTypeOneOf2
     */
    regex: string;
}

/**
 * Check if a given object implements the SearchTypeOneOf2 interface.
 */
export function instanceOfSearchTypeOneOf2(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "regex" in value;

    return isInstance;
}

export function SearchTypeOneOf2FromJSON(json: any): SearchTypeOneOf2 {
    return SearchTypeOneOf2FromJSONTyped(json, false);
}

export function SearchTypeOneOf2FromJSONTyped(json: any, ignoreDiscriminator: boolean): SearchTypeOneOf2 {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'regex': json['Regex'],
    };
}

export function SearchTypeOneOf2ToJSON(value?: SearchTypeOneOf2 | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'Regex': value.regex,
    };
}

