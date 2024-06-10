"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""

import abc
import grpc
import nucliadb_telemetry.tests.grpc.helloworld_pb2

class GreeterStub:
    """The greeting service definition."""

    def __init__(self, channel: grpc.Channel) -> None: ...
    SayHello: grpc.UnaryUnaryMultiCallable[
        nucliadb_telemetry.tests.grpc.helloworld_pb2.HelloRequest,
        nucliadb_telemetry.tests.grpc.helloworld_pb2.HelloReply,
    ]
    """Sends a greeting"""

class GreeterServicer(metaclass=abc.ABCMeta):
    """The greeting service definition."""

    @abc.abstractmethod
    def SayHello(
        self,
        request: nucliadb_telemetry.tests.grpc.helloworld_pb2.HelloRequest,
        context: grpc.ServicerContext,
    ) -> nucliadb_telemetry.tests.grpc.helloworld_pb2.HelloReply:
        """Sends a greeting"""
        pass

def add_GreeterServicer_to_server(
    servicer: GreeterServicer, server: grpc.Server
) -> None: ...