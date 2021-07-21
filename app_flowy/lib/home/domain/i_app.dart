import 'package:flowy_sdk/protobuf/flowy-workspace/protobuf.dart';
import 'package:dartz/dartz.dart';

abstract class IApp {
  Future<Either<List<View>, WorkspaceError>> getViews({required String appId});

  Future<Either<View, WorkspaceError>> createView(
      {required String appId,
      required String name,
      String? desc,
      required ViewTypeIdentifier viewType});
}
