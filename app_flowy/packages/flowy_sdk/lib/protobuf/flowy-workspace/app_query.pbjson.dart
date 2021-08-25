///
//  Generated code. Do not modify.
//  source: app_query.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use queryAppRequestDescriptor instead')
const QueryAppRequest$json = const {
  '1': 'QueryAppRequest',
  '2': const [
    const {'1': 'app_id', '3': 1, '4': 1, '5': 9, '10': 'appId'},
    const {'1': 'read_belongings', '3': 2, '4': 1, '5': 8, '10': 'readBelongings'},
    const {'1': 'is_trash', '3': 3, '4': 1, '5': 8, '10': 'isTrash'},
  ],
};

/// Descriptor for `QueryAppRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List queryAppRequestDescriptor = $convert.base64Decode('Cg9RdWVyeUFwcFJlcXVlc3QSFQoGYXBwX2lkGAEgASgJUgVhcHBJZBInCg9yZWFkX2JlbG9uZ2luZ3MYAiABKAhSDnJlYWRCZWxvbmdpbmdzEhkKCGlzX3RyYXNoGAMgASgIUgdpc1RyYXNo');
@$core.Deprecated('Use queryAppParamsDescriptor instead')
const QueryAppParams$json = const {
  '1': 'QueryAppParams',
  '2': const [
    const {'1': 'app_id', '3': 1, '4': 1, '5': 9, '10': 'appId'},
    const {'1': 'read_belongings', '3': 2, '4': 1, '5': 8, '10': 'readBelongings'},
    const {'1': 'is_trash', '3': 3, '4': 1, '5': 8, '10': 'isTrash'},
  ],
};

/// Descriptor for `QueryAppParams`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List queryAppParamsDescriptor = $convert.base64Decode('Cg5RdWVyeUFwcFBhcmFtcxIVCgZhcHBfaWQYASABKAlSBWFwcElkEicKD3JlYWRfYmVsb25naW5ncxgCIAEoCFIOcmVhZEJlbG9uZ2luZ3MSGQoIaXNfdHJhc2gYAyABKAhSB2lzVHJhc2g=');
