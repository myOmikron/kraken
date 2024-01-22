/* tslint:disable */
/* eslint-disable */


import {
    WsMessageOneOf,
    WsMessageOneOfTypeEnum,
    WsMessageOneOfFromJSONTyped,
    WsMessageOneOfToJSON,
} from './WsMessageOneOf';
import {
    WsMessageOneOf1,
    WsMessageOneOf1TypeEnum,
    WsMessageOneOf1FromJSONTyped,
    WsMessageOneOf1ToJSON,
} from './WsMessageOneOf1';
import {
    WsMessageOneOf10,
    WsMessageOneOf10TypeEnum,
    WsMessageOneOf10FromJSONTyped,
    WsMessageOneOf10ToJSON,
} from './WsMessageOneOf10';
import {
    WsMessageOneOf11,
    WsMessageOneOf11TypeEnum,
    WsMessageOneOf11FromJSONTyped,
    WsMessageOneOf11ToJSON,
} from './WsMessageOneOf11';
import {
    WsMessageOneOf12,
    WsMessageOneOf12TypeEnum,
    WsMessageOneOf12FromJSONTyped,
    WsMessageOneOf12ToJSON,
} from './WsMessageOneOf12';
import {
    WsMessageOneOf13,
    WsMessageOneOf13TypeEnum,
    WsMessageOneOf13FromJSONTyped,
    WsMessageOneOf13ToJSON,
} from './WsMessageOneOf13';
import {
    WsMessageOneOf14,
    WsMessageOneOf14TypeEnum,
    WsMessageOneOf14FromJSONTyped,
    WsMessageOneOf14ToJSON,
} from './WsMessageOneOf14';
import {
    WsMessageOneOf15,
    WsMessageOneOf15TypeEnum,
    WsMessageOneOf15FromJSONTyped,
    WsMessageOneOf15ToJSON,
} from './WsMessageOneOf15';
import {
    WsMessageOneOf16,
    WsMessageOneOf16TypeEnum,
    WsMessageOneOf16FromJSONTyped,
    WsMessageOneOf16ToJSON,
} from './WsMessageOneOf16';
import {
    WsMessageOneOf17,
    WsMessageOneOf17TypeEnum,
    WsMessageOneOf17FromJSONTyped,
    WsMessageOneOf17ToJSON,
} from './WsMessageOneOf17';
import {
    WsMessageOneOf18,
    WsMessageOneOf18TypeEnum,
    WsMessageOneOf18FromJSONTyped,
    WsMessageOneOf18ToJSON,
} from './WsMessageOneOf18';
import {
    WsMessageOneOf19,
    WsMessageOneOf19TypeEnum,
    WsMessageOneOf19FromJSONTyped,
    WsMessageOneOf19ToJSON,
} from './WsMessageOneOf19';
import {
    WsMessageOneOf2,
    WsMessageOneOf2TypeEnum,
    WsMessageOneOf2FromJSONTyped,
    WsMessageOneOf2ToJSON,
} from './WsMessageOneOf2';
import {
    WsMessageOneOf20,
    WsMessageOneOf20TypeEnum,
    WsMessageOneOf20FromJSONTyped,
    WsMessageOneOf20ToJSON,
} from './WsMessageOneOf20';
import {
    WsMessageOneOf21,
    WsMessageOneOf21TypeEnum,
    WsMessageOneOf21FromJSONTyped,
    WsMessageOneOf21ToJSON,
} from './WsMessageOneOf21';
import {
    WsMessageOneOf22,
    WsMessageOneOf22TypeEnum,
    WsMessageOneOf22FromJSONTyped,
    WsMessageOneOf22ToJSON,
} from './WsMessageOneOf22';
import {
    WsMessageOneOf3,
    WsMessageOneOf3TypeEnum,
    WsMessageOneOf3FromJSONTyped,
    WsMessageOneOf3ToJSON,
} from './WsMessageOneOf3';
import {
    WsMessageOneOf4,
    WsMessageOneOf4TypeEnum,
    WsMessageOneOf4FromJSONTyped,
    WsMessageOneOf4ToJSON,
} from './WsMessageOneOf4';
import {
    WsMessageOneOf5,
    WsMessageOneOf5TypeEnum,
    WsMessageOneOf5FromJSONTyped,
    WsMessageOneOf5ToJSON,
} from './WsMessageOneOf5';
import {
    WsMessageOneOf6,
    WsMessageOneOf6TypeEnum,
    WsMessageOneOf6FromJSONTyped,
    WsMessageOneOf6ToJSON,
} from './WsMessageOneOf6';
import {
    WsMessageOneOf7,
    WsMessageOneOf7TypeEnum,
    WsMessageOneOf7FromJSONTyped,
    WsMessageOneOf7ToJSON,
} from './WsMessageOneOf7';
import {
    WsMessageOneOf8,
    WsMessageOneOf8TypeEnum,
    WsMessageOneOf8FromJSONTyped,
    WsMessageOneOf8ToJSON,
} from './WsMessageOneOf8';
import {
    WsMessageOneOf9,
    WsMessageOneOf9TypeEnum,
    WsMessageOneOf9FromJSONTyped,
    WsMessageOneOf9ToJSON,
} from './WsMessageOneOf9';

/**
 * @type WsMessage
 * Message that is sent via websocket
 * @export
 */
export type WsMessage = 
  | WsMessageOneOf
  | WsMessageOneOf1
  | WsMessageOneOf10
  | WsMessageOneOf11
  | WsMessageOneOf12
  | WsMessageOneOf13
  | WsMessageOneOf14
  | WsMessageOneOf15
  | WsMessageOneOf16
  | WsMessageOneOf17
  | WsMessageOneOf18
  | WsMessageOneOf19
  | WsMessageOneOf2
  | WsMessageOneOf20
  | WsMessageOneOf21
  | WsMessageOneOf22
  | WsMessageOneOf3
  | WsMessageOneOf4
  | WsMessageOneOf5
  | WsMessageOneOf6
  | WsMessageOneOf7
  | WsMessageOneOf8
  | WsMessageOneOf9;

function enumToString<T extends string>(obj: Record<T, T>): T {
    // @ts-ignore
    return Object.values(obj)[0];
}

const WsMessageOneOfType = enumToString(WsMessageOneOfTypeEnum);
const WsMessageOneOf1Type = enumToString(WsMessageOneOf1TypeEnum);
const WsMessageOneOf10Type = enumToString(WsMessageOneOf10TypeEnum);
const WsMessageOneOf11Type = enumToString(WsMessageOneOf11TypeEnum);
const WsMessageOneOf12Type = enumToString(WsMessageOneOf12TypeEnum);
const WsMessageOneOf13Type = enumToString(WsMessageOneOf13TypeEnum);
const WsMessageOneOf14Type = enumToString(WsMessageOneOf14TypeEnum);
const WsMessageOneOf15Type = enumToString(WsMessageOneOf15TypeEnum);
const WsMessageOneOf16Type = enumToString(WsMessageOneOf16TypeEnum);
const WsMessageOneOf17Type = enumToString(WsMessageOneOf17TypeEnum);
const WsMessageOneOf18Type = enumToString(WsMessageOneOf18TypeEnum);
const WsMessageOneOf19Type = enumToString(WsMessageOneOf19TypeEnum);
const WsMessageOneOf2Type = enumToString(WsMessageOneOf2TypeEnum);
const WsMessageOneOf20Type = enumToString(WsMessageOneOf20TypeEnum);
const WsMessageOneOf21Type = enumToString(WsMessageOneOf21TypeEnum);
const WsMessageOneOf22Type = enumToString(WsMessageOneOf22TypeEnum);
const WsMessageOneOf3Type = enumToString(WsMessageOneOf3TypeEnum);
const WsMessageOneOf4Type = enumToString(WsMessageOneOf4TypeEnum);
const WsMessageOneOf5Type = enumToString(WsMessageOneOf5TypeEnum);
const WsMessageOneOf6Type = enumToString(WsMessageOneOf6TypeEnum);
const WsMessageOneOf7Type = enumToString(WsMessageOneOf7TypeEnum);
const WsMessageOneOf8Type = enumToString(WsMessageOneOf8TypeEnum);
const WsMessageOneOf9Type = enumToString(WsMessageOneOf9TypeEnum);

export function WsMessageFromJSON(json: any): WsMessage {
    return WsMessageFromJSONTyped(json, false);
}

export function WsMessageFromJSONTyped(json: any, ignoreDiscriminator: boolean): WsMessage {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    switch (json['type']) {
        
        case WsMessageOneOfType:
            return WsMessageOneOfFromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf1Type:
            return WsMessageOneOf1FromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf10Type:
            return WsMessageOneOf10FromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf11Type:
            return WsMessageOneOf11FromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf12Type:
            return WsMessageOneOf12FromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf13Type:
            return WsMessageOneOf13FromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf14Type:
            return WsMessageOneOf14FromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf15Type:
            return WsMessageOneOf15FromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf16Type:
            return WsMessageOneOf16FromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf17Type:
            return WsMessageOneOf17FromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf18Type:
            return WsMessageOneOf18FromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf19Type:
            return WsMessageOneOf19FromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf2Type:
            return WsMessageOneOf2FromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf20Type:
            return WsMessageOneOf20FromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf21Type:
            return WsMessageOneOf21FromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf22Type:
            return WsMessageOneOf22FromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf3Type:
            return WsMessageOneOf3FromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf4Type:
            return WsMessageOneOf4FromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf5Type:
            return WsMessageOneOf5FromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf6Type:
            return WsMessageOneOf6FromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf7Type:
            return WsMessageOneOf7FromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf8Type:
            return WsMessageOneOf8FromJSONTyped(json, ignoreDiscriminator);
        case WsMessageOneOf9Type:
            return WsMessageOneOf9FromJSONTyped(json, ignoreDiscriminator);
        default:
            throw new Error("No variant of WsMessage exists with 'type=" + json["type"] + "'");
    }
}

export function WsMessageToJSON(value?: WsMessage | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    switch (value['type']) {
        
        case WsMessageOneOfType:
            return WsMessageOneOfToJSON(value);
        case WsMessageOneOf1Type:
            return WsMessageOneOf1ToJSON(value);
        case WsMessageOneOf10Type:
            return WsMessageOneOf10ToJSON(value);
        case WsMessageOneOf11Type:
            return WsMessageOneOf11ToJSON(value);
        case WsMessageOneOf12Type:
            return WsMessageOneOf12ToJSON(value);
        case WsMessageOneOf13Type:
            return WsMessageOneOf13ToJSON(value);
        case WsMessageOneOf14Type:
            return WsMessageOneOf14ToJSON(value);
        case WsMessageOneOf15Type:
            return WsMessageOneOf15ToJSON(value);
        case WsMessageOneOf16Type:
            return WsMessageOneOf16ToJSON(value);
        case WsMessageOneOf17Type:
            return WsMessageOneOf17ToJSON(value);
        case WsMessageOneOf18Type:
            return WsMessageOneOf18ToJSON(value);
        case WsMessageOneOf19Type:
            return WsMessageOneOf19ToJSON(value);
        case WsMessageOneOf2Type:
            return WsMessageOneOf2ToJSON(value);
        case WsMessageOneOf20Type:
            return WsMessageOneOf20ToJSON(value);
        case WsMessageOneOf21Type:
            return WsMessageOneOf21ToJSON(value);
        case WsMessageOneOf22Type:
            return WsMessageOneOf22ToJSON(value);
        case WsMessageOneOf3Type:
            return WsMessageOneOf3ToJSON(value);
        case WsMessageOneOf4Type:
            return WsMessageOneOf4ToJSON(value);
        case WsMessageOneOf5Type:
            return WsMessageOneOf5ToJSON(value);
        case WsMessageOneOf6Type:
            return WsMessageOneOf6ToJSON(value);
        case WsMessageOneOf7Type:
            return WsMessageOneOf7ToJSON(value);
        case WsMessageOneOf8Type:
            return WsMessageOneOf8ToJSON(value);
        case WsMessageOneOf9Type:
            return WsMessageOneOf9ToJSON(value);
        default:
            throw new Error("No variant of WsMessage exists with 'type=" + value["type"] + "'");
    }

}
