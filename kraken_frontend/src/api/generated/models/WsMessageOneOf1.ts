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
import type { SimpleUser } from './SimpleUser';
import {
    SimpleUserFromJSON,
    SimpleUserFromJSONTyped,
    SimpleUserToJSON,
} from './SimpleUser';
import type { SimpleWorkspace } from './SimpleWorkspace';
import {
    SimpleWorkspaceFromJSON,
    SimpleWorkspaceFromJSONTyped,
    SimpleWorkspaceToJSON,
} from './SimpleWorkspace';

/**
 * An invitation to a workspace was issued
 * @export
 * @interface WsMessageOneOf1
 */
export interface WsMessageOneOf1 {
    /**
     * The uuid of the invitation
     * @type {string}
     * @memberof WsMessageOneOf1
     */
    invitationUuid: string;
    /**
     * 
     * @type {SimpleWorkspace}
     * @memberof WsMessageOneOf1
     */
    workspace: SimpleWorkspace;
    /**
     * 
     * @type {SimpleUser}
     * @memberof WsMessageOneOf1
     */
    from: SimpleUser;
    /**
     * 
     * @type {string}
     * @memberof WsMessageOneOf1
     */
    type: WsMessageOneOf1TypeEnum;
}


/**
 * @export
 */
export const WsMessageOneOf1TypeEnum = {
    InvitationToWorkspace: 'InvitationToWorkspace'
} as const;
export type WsMessageOneOf1TypeEnum = typeof WsMessageOneOf1TypeEnum[keyof typeof WsMessageOneOf1TypeEnum];


/**
 * Check if a given object implements the WsMessageOneOf1 interface.
 */
export function instanceOfWsMessageOneOf1(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "invitationUuid" in value;
    isInstance = isInstance && "workspace" in value;
    isInstance = isInstance && "from" in value;
    isInstance = isInstance && "type" in value;

    return isInstance;
}

export function WsMessageOneOf1FromJSON(json: any): WsMessageOneOf1 {
    return WsMessageOneOf1FromJSONTyped(json, false);
}

export function WsMessageOneOf1FromJSONTyped(json: any, ignoreDiscriminator: boolean): WsMessageOneOf1 {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'invitationUuid': json['invitation_uuid'],
        'workspace': SimpleWorkspaceFromJSON(json['workspace']),
        'from': SimpleUserFromJSON(json['from']),
        'type': json['type'],
    };
}

export function WsMessageOneOf1ToJSON(value?: WsMessageOneOf1 | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'invitation_uuid': value.invitationUuid,
        'workspace': SimpleWorkspaceToJSON(value.workspace),
        'from': SimpleUserToJSON(value.from),
        'type': value.type,
    };
}

