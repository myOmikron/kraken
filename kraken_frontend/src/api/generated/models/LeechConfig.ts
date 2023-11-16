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
 * The configuration of a leech
 * @export
 * @interface LeechConfig
 */
export interface LeechConfig {
    /**
     * PEM encoded CA managed by kraken
     * @type {string}
     * @memberof LeechConfig
     */
    ca: string;
    /**
     * PEM encoded certificate
     * @type {string}
     * @memberof LeechConfig
     */
    cert: string;
    /**
     * PEM encoded private key for the certificate
     * @type {string}
     * @memberof LeechConfig
     */
    key: string;
    /**
     * The randomly generated fake domain for the kraken to be used for sni
     * @type {string}
     * @memberof LeechConfig
     */
    sni: string;
    /**
     * 
     * @type {string}
     * @memberof LeechConfig
     */
    secret: string;
}

/**
 * Check if a given object implements the LeechConfig interface.
 */
export function instanceOfLeechConfig(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "ca" in value;
    isInstance = isInstance && "cert" in value;
    isInstance = isInstance && "key" in value;
    isInstance = isInstance && "sni" in value;
    isInstance = isInstance && "secret" in value;

    return isInstance;
}

export function LeechConfigFromJSON(json: any): LeechConfig {
    return LeechConfigFromJSONTyped(json, false);
}

export function LeechConfigFromJSONTyped(json: any, ignoreDiscriminator: boolean): LeechConfig {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'ca': json['ca'],
        'cert': json['cert'],
        'key': json['key'],
        'sni': json['sni'],
        'secret': json['secret'],
    };
}

export function LeechConfigToJSON(value?: LeechConfig | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'ca': value.ca,
        'cert': value.cert,
        'key': value.key,
        'sni': value.sni,
        'secret': value.secret,
    };
}
