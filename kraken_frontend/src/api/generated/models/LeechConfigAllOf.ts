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
 * @interface LeechConfigAllOf
 */
export interface LeechConfigAllOf {
    /**
     * The secret of the leech
     * @type {string}
     * @memberof LeechConfigAllOf
     */
    secret: string;
}

/**
 * Check if a given object implements the LeechConfigAllOf interface.
 */
export function instanceOfLeechConfigAllOf(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "secret" in value;

    return isInstance;
}

export function LeechConfigAllOfFromJSON(json: any): LeechConfigAllOf {
    return LeechConfigAllOfFromJSONTyped(json, false);
}

export function LeechConfigAllOfFromJSONTyped(json: any, ignoreDiscriminator: boolean): LeechConfigAllOf {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'secret': json['secret'],
    };
}

export function LeechConfigAllOfToJSON(value?: LeechConfigAllOf | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'secret': value.secret,
    };
}

