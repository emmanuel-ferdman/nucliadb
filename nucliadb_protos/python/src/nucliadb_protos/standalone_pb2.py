# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: nucliadb_protos/standalone.proto
# Protobuf Python Version: 4.25.1
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n nucliadb_protos/standalone.proto\x12\nstandalone\"E\n\x11NodeActionRequest\x12\x0f\n\x07service\x18\x01 \x01(\t\x12\x0e\n\x06\x61\x63tion\x18\x02 \x01(\t\x12\x0f\n\x07payload\x18\x03 \x01(\x0c\"%\n\x12NodeActionResponse\x12\x0f\n\x07payload\x18\x01 \x01(\x0c\"\x11\n\x0fNodeInfoRequest\"p\n\x10NodeInfoResponse\x12\n\n\x02id\x18\x01 \x01(\t\x12\x0f\n\x07\x61\x64\x64ress\x18\x02 \x01(\t\x12\x13\n\x0bshard_count\x18\x03 \x01(\r\x12\x16\n\x0e\x61vailable_disk\x18\x04 \x01(\x04\x12\x12\n\ntotal_disk\x18\x05 \x01(\x04\x32\xb2\x01\n\x18StandaloneClusterService\x12M\n\nNodeAction\x12\x1d.standalone.NodeActionRequest\x1a\x1e.standalone.NodeActionResponse\"\x00\x12G\n\x08NodeInfo\x12\x1b.standalone.NodeInfoRequest\x1a\x1c.standalone.NodeInfoResponse\"\x00\x62\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'nucliadb_protos.standalone_pb2', _globals)
if _descriptor._USE_C_DESCRIPTORS == False:
  DESCRIPTOR._options = None
  _globals['_NODEACTIONREQUEST']._serialized_start=48
  _globals['_NODEACTIONREQUEST']._serialized_end=117
  _globals['_NODEACTIONRESPONSE']._serialized_start=119
  _globals['_NODEACTIONRESPONSE']._serialized_end=156
  _globals['_NODEINFOREQUEST']._serialized_start=158
  _globals['_NODEINFOREQUEST']._serialized_end=175
  _globals['_NODEINFORESPONSE']._serialized_start=177
  _globals['_NODEINFORESPONSE']._serialized_end=289
  _globals['_STANDALONECLUSTERSERVICE']._serialized_start=292
  _globals['_STANDALONECLUSTERSERVICE']._serialized_end=470
# @@protoc_insertion_point(module_scope)