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
import type { SimpleDnsResolutionResult } from './SimpleDnsResolutionResult';
import {
    SimpleDnsResolutionResultFromJSON,
    SimpleDnsResolutionResultFromJSONTyped,
    SimpleDnsResolutionResultToJSON,
} from './SimpleDnsResolutionResult';

/**
 * 
 * @export
 * @interface SourceAttackResultOneOf7
 */
export interface SourceAttackResultOneOf7 {
    /**
     * 
     * @type {string}
     * @memberof SourceAttackResultOneOf7
     */
    attackType: SourceAttackResultOneOf7AttackTypeEnum;
    /**
     * The [`AttackType::DnsResolution`] and its results
     * @type {Array<SimpleDnsResolutionResult>}
     * @memberof SourceAttackResultOneOf7
     */
    results: Array<SimpleDnsResolutionResult>;
}


/**
 * @export
 */
export const SourceAttackResultOneOf7AttackTypeEnum = {
    DnsResolution: 'DnsResolution'
} as const;
export type SourceAttackResultOneOf7AttackTypeEnum = typeof SourceAttackResultOneOf7AttackTypeEnum[keyof typeof SourceAttackResultOneOf7AttackTypeEnum];


/**
 * Check if a given object implements the SourceAttackResultOneOf7 interface.
 */
export function instanceOfSourceAttackResultOneOf7(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "attackType" in value;
    isInstance = isInstance && "results" in value;

    return isInstance;
}

export function SourceAttackResultOneOf7FromJSON(json: any): SourceAttackResultOneOf7 {
    return SourceAttackResultOneOf7FromJSONTyped(json, false);
}

export function SourceAttackResultOneOf7FromJSONTyped(json: any, ignoreDiscriminator: boolean): SourceAttackResultOneOf7 {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'attackType': json['attack_type'],
        'results': ((json['results'] as Array<any>).map(SimpleDnsResolutionResultFromJSON)),
    };
}

export function SourceAttackResultOneOf7ToJSON(value?: SourceAttackResultOneOf7 | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'attack_type': value.attackType,
        'results': ((value.results as Array<any>).map(SimpleDnsResolutionResultToJSON)),
    };
}

