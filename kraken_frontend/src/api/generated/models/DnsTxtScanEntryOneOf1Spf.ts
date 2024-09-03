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
import type { DnsTxtScanSpfType } from './DnsTxtScanSpfType';
import {
    DnsTxtScanSpfTypeFromJSON,
    DnsTxtScanSpfTypeFromJSONTyped,
    DnsTxtScanSpfTypeToJSON,
} from './DnsTxtScanSpfType';

/**
 * Wraps a single SPF rule part, the rule is reconstructed from the parsed value
 * @export
 * @interface DnsTxtScanEntryOneOf1Spf
 */
export interface DnsTxtScanEntryOneOf1Spf {
    /**
     * The primary key
     * @type {string}
     * @memberof DnsTxtScanEntryOneOf1Spf
     */
    uuid: string;
    /**
     * The point in time, this entry was produced
     * @type {Date}
     * @memberof DnsTxtScanEntryOneOf1Spf
     */
    createdAt: Date;
    /**
     * A single SPF rule part that was matched for this object.
     * @type {string}
     * @memberof DnsTxtScanEntryOneOf1Spf
     */
    rule: string;
    /**
     * 
     * @type {DnsTxtScanSpfType}
     * @memberof DnsTxtScanEntryOneOf1Spf
     */
    spfType: DnsTxtScanSpfType;
    /**
     * If the txt_type is a SPF type that includes an IP (or whole IP range), it will be set here.
     * @type {string}
     * @memberof DnsTxtScanEntryOneOf1Spf
     */
    spfIp: string;
    /**
     * If the txt_type is a SPF type that includes a domain, it will be set here.
     * @type {string}
     * @memberof DnsTxtScanEntryOneOf1Spf
     */
    spfDomain?: string | null;
    /**
     * If the txt_type is a SPF type that includes a domain, this is its ipv4 CIDR.
     * @type {number}
     * @memberof DnsTxtScanEntryOneOf1Spf
     */
    spfDomainIpv4Cidr?: number | null;
    /**
     * If the txt_type is a SPF type that includes a domain, this is its ipv6 CIDR.
     * @type {number}
     * @memberof DnsTxtScanEntryOneOf1Spf
     */
    spfDomainIpv6Cidr?: number | null;
}

/**
 * Check if a given object implements the DnsTxtScanEntryOneOf1Spf interface.
 */
export function instanceOfDnsTxtScanEntryOneOf1Spf(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "uuid" in value;
    isInstance = isInstance && "createdAt" in value;
    isInstance = isInstance && "rule" in value;
    isInstance = isInstance && "spfType" in value;
    isInstance = isInstance && "spfIp" in value;

    return isInstance;
}

export function DnsTxtScanEntryOneOf1SpfFromJSON(json: any): DnsTxtScanEntryOneOf1Spf {
    return DnsTxtScanEntryOneOf1SpfFromJSONTyped(json, false);
}

export function DnsTxtScanEntryOneOf1SpfFromJSONTyped(json: any, ignoreDiscriminator: boolean): DnsTxtScanEntryOneOf1Spf {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'uuid': json['uuid'],
        'createdAt': (new Date(json['created_at'])),
        'rule': json['rule'],
        'spfType': DnsTxtScanSpfTypeFromJSON(json['spf_type']),
        'spfIp': json['spf_ip'],
        'spfDomain': !exists(json, 'spf_domain') ? undefined : json['spf_domain'],
        'spfDomainIpv4Cidr': !exists(json, 'spf_domain_ipv4_cidr') ? undefined : json['spf_domain_ipv4_cidr'],
        'spfDomainIpv6Cidr': !exists(json, 'spf_domain_ipv6_cidr') ? undefined : json['spf_domain_ipv6_cidr'],
    };
}

export function DnsTxtScanEntryOneOf1SpfToJSON(value?: DnsTxtScanEntryOneOf1Spf | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'uuid': value.uuid,
        'created_at': (value.createdAt.toISOString()),
        'rule': value.rule,
        'spf_type': DnsTxtScanSpfTypeToJSON(value.spfType),
        'spf_ip': value.spfIp,
        'spf_domain': value.spfDomain,
        'spf_domain_ipv4_cidr': value.spfDomainIpv4Cidr,
        'spf_domain_ipv6_cidr': value.spfDomainIpv6Cidr,
    };
}

