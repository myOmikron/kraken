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
/**
 * Entry of certificate transparency results
 * @export
 * @interface CertificateTransparencyEntry
 */
export interface CertificateTransparencyEntry {
    /**
     * The serial number of the certificate
     * @type {string}
     * @memberof CertificateTransparencyEntry
     */
    serialNumber: string;
    /**
     * The name of the issuer for the certificate
     * @type {string}
     * @memberof CertificateTransparencyEntry
     */
    issuerName: string;
    /**
     * The common name of the certificate
     * @type {string}
     * @memberof CertificateTransparencyEntry
     */
    commonName: string;
    /**
     * The value names of the certificate
     * @type {Array<string>}
     * @memberof CertificateTransparencyEntry
     */
    valueNames: Array<string>;
    /**
     * The point in time after the certificate is valid
     * @type {Date}
     * @memberof CertificateTransparencyEntry
     */
    notBefore?: Date | null;
    /**
     * The point in time before the certificate is valid
     * @type {Date}
     * @memberof CertificateTransparencyEntry
     */
    notAfter?: Date | null;
}

/**
 * Check if a given object implements the CertificateTransparencyEntry interface.
 */
export function instanceOfCertificateTransparencyEntry(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "serialNumber" in value;
    isInstance = isInstance && "issuerName" in value;
    isInstance = isInstance && "commonName" in value;
    isInstance = isInstance && "valueNames" in value;

    return isInstance;
}

export function CertificateTransparencyEntryFromJSON(json: any): CertificateTransparencyEntry {
    return CertificateTransparencyEntryFromJSONTyped(json, false);
}

export function CertificateTransparencyEntryFromJSONTyped(json: any, ignoreDiscriminator: boolean): CertificateTransparencyEntry {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'serialNumber': json['serial_number'],
        'issuerName': json['issuer_name'],
        'commonName': json['common_name'],
        'valueNames': json['value_names'],
        'notBefore': !exists(json, 'not_before') ? undefined : (json['not_before'] === null ? null : new Date(json['not_before'])),
        'notAfter': !exists(json, 'not_after') ? undefined : (json['not_after'] === null ? null : new Date(json['not_after'])),
    };
}

export function CertificateTransparencyEntryToJSON(value?: CertificateTransparencyEntry | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'serial_number': value.serialNumber,
        'issuer_name': value.issuerName,
        'common_name': value.commonName,
        'value_names': value.valueNames,
        'not_before': value.notBefore === undefined ? undefined : (value.notBefore === null ? null : value.notBefore.toISOString()),
        'not_after': value.notAfter === undefined ? undefined : (value.notAfter === null ? null : value.notAfter.toISOString()),
    };
}

