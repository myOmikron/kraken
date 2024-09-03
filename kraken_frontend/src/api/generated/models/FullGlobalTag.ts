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
import type { Color } from './Color';
import {
    ColorFromJSON,
    ColorFromJSONTyped,
    ColorToJSON,
} from './Color';

/**
 * The full representation of a full
 * @export
 * @interface FullGlobalTag
 */
export interface FullGlobalTag {
    /**
     * The uuid of the tag
     * @type {string}
     * @memberof FullGlobalTag
     */
    uuid: string;
    /**
     * The name of the tag
     * @type {string}
     * @memberof FullGlobalTag
     */
    name: string;
    /**
     * 
     * @type {Color}
     * @memberof FullGlobalTag
     */
    color: Color;
}

/**
 * Check if a given object implements the FullGlobalTag interface.
 */
export function instanceOfFullGlobalTag(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "uuid" in value;
    isInstance = isInstance && "name" in value;
    isInstance = isInstance && "color" in value;

    return isInstance;
}

export function FullGlobalTagFromJSON(json: any): FullGlobalTag {
    return FullGlobalTagFromJSONTyped(json, false);
}

export function FullGlobalTagFromJSONTyped(json: any, ignoreDiscriminator: boolean): FullGlobalTag {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'uuid': json['uuid'],
        'name': json['name'],
        'color': ColorFromJSON(json['color']),
    };
}

export function FullGlobalTagToJSON(value?: FullGlobalTag | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'uuid': value.uuid,
        'name': value.name,
        'color': ColorToJSON(value.color),
    };
}

