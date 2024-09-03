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
import type { SimpleBruteforceSubdomainsResult } from './SimpleBruteforceSubdomainsResult';
import {
    SimpleBruteforceSubdomainsResultFromJSON,
    SimpleBruteforceSubdomainsResultFromJSONTyped,
    SimpleBruteforceSubdomainsResultToJSON,
} from './SimpleBruteforceSubdomainsResult';

/**
 * 
 * @export
 * @interface SourceAttackResultOneOf
 */
export interface SourceAttackResultOneOf {
    /**
     * 
     * @type {string}
     * @memberof SourceAttackResultOneOf
     */
    attackType: SourceAttackResultOneOfAttackTypeEnum;
    /**
     * The [`AttackType::BruteforceSubdomains`] and its results
     * @type {Array<SimpleBruteforceSubdomainsResult>}
     * @memberof SourceAttackResultOneOf
     */
    results: Array<SimpleBruteforceSubdomainsResult>;
}


/**
 * @export
 */
export const SourceAttackResultOneOfAttackTypeEnum = {
    BruteforceSubdomains: 'BruteforceSubdomains'
} as const;
export type SourceAttackResultOneOfAttackTypeEnum = typeof SourceAttackResultOneOfAttackTypeEnum[keyof typeof SourceAttackResultOneOfAttackTypeEnum];


/**
 * Check if a given object implements the SourceAttackResultOneOf interface.
 */
export function instanceOfSourceAttackResultOneOf(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "attackType" in value;
    isInstance = isInstance && "results" in value;

    return isInstance;
}

export function SourceAttackResultOneOfFromJSON(json: any): SourceAttackResultOneOf {
    return SourceAttackResultOneOfFromJSONTyped(json, false);
}

export function SourceAttackResultOneOfFromJSONTyped(json: any, ignoreDiscriminator: boolean): SourceAttackResultOneOf {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'attackType': json['attack_type'],
        'results': ((json['results'] as Array<any>).map(SimpleBruteforceSubdomainsResultFromJSON)),
    };
}

export function SourceAttackResultOneOfToJSON(value?: SourceAttackResultOneOf | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'attack_type': value.attackType,
        'results': ((value.results as Array<any>).map(SimpleBruteforceSubdomainsResultToJSON)),
    };
}

