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
import type { SimpleHttpService } from './SimpleHttpService';
import {
    SimpleHttpServiceFromJSON,
    SimpleHttpServiceFromJSONTyped,
    SimpleHttpServiceToJSON,
} from './SimpleHttpService';

/**
 * 
 * @export
 * @interface FindingAffectedObjectOneOf4
 */
export interface FindingAffectedObjectOneOf4 {
    /**
     * 
     * @type {SimpleHttpService}
     * @memberof FindingAffectedObjectOneOf4
     */
    httpService: SimpleHttpService;
}

/**
 * Check if a given object implements the FindingAffectedObjectOneOf4 interface.
 */
export function instanceOfFindingAffectedObjectOneOf4(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "httpService" in value;

    return isInstance;
}

export function FindingAffectedObjectOneOf4FromJSON(json: any): FindingAffectedObjectOneOf4 {
    return FindingAffectedObjectOneOf4FromJSONTyped(json, false);
}

export function FindingAffectedObjectOneOf4FromJSONTyped(json: any, ignoreDiscriminator: boolean): FindingAffectedObjectOneOf4 {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'httpService': SimpleHttpServiceFromJSON(json['HttpService']),
    };
}

export function FindingAffectedObjectOneOf4ToJSON(value?: FindingAffectedObjectOneOf4 | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'HttpService': SimpleHttpServiceToJSON(value.httpService),
    };
}

