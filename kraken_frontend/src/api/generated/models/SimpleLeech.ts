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
 * The simple representation of a leech
 * @export
 * @interface SimpleLeech
 */
export interface SimpleLeech {
    /**
     * uuid of the leech
     * @type {string}
     * @memberof SimpleLeech
     */
    uuid: string;
    /**
     * Name of the leech
     * @type {string}
     * @memberof SimpleLeech
     */
    name: string;
    /**
     * Address of the leech
     * @type {string}
     * @memberof SimpleLeech
     */
    address: string;
}

/**
 * Check if a given object implements the SimpleLeech interface.
 */
export function instanceOfSimpleLeech(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "uuid" in value;
    isInstance = isInstance && "name" in value;
    isInstance = isInstance && "address" in value;

    return isInstance;
}

export function SimpleLeechFromJSON(json: any): SimpleLeech {
    return SimpleLeechFromJSONTyped(json, false);
}

export function SimpleLeechFromJSONTyped(json: any, ignoreDiscriminator: boolean): SimpleLeech {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'uuid': json['uuid'],
        'name': json['name'],
        'address': json['address'],
    };
}

export function SimpleLeechToJSON(value?: SimpleLeech | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'uuid': value.uuid,
        'name': value.name,
        'address': value.address,
    };
}

