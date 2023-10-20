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
 * A representation of a full api key
 * @export
 * @interface FullApiKey
 */
export interface FullApiKey {
    /**
     * The key's identifier
     * @type {string}
     * @memberof FullApiKey
     */
    uuid: string;
    /**
     * A descriptive name helping the user to identify the key
     * @type {string}
     * @memberof FullApiKey
     */
    name: string;
    /**
     * The actual key's value
     * @type {string}
     * @memberof FullApiKey
     */
    key: string;
}

/**
 * Check if a given object implements the FullApiKey interface.
 */
export function instanceOfFullApiKey(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "uuid" in value;
    isInstance = isInstance && "name" in value;
    isInstance = isInstance && "key" in value;

    return isInstance;
}

export function FullApiKeyFromJSON(json: any): FullApiKey {
    return FullApiKeyFromJSONTyped(json, false);
}

export function FullApiKeyFromJSONTyped(json: any, ignoreDiscriminator: boolean): FullApiKey {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'uuid': json['uuid'],
        'name': json['name'],
        'key': json['key'],
    };
}

export function FullApiKeyToJSON(value?: FullApiKey | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'uuid': value.uuid,
        'name': value.name,
        'key': value.key,
    };
}
