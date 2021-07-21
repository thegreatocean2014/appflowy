import 'package:flowy_sdk/protobuf/flowy-workspace/protobuf.dart';
import 'package:dartz/dartz.dart';

typedef AppUpdatedCallback = void Function(String name, String desc);
typedef AppAddViewCallback = void Function(
    Either<List<View>, WorkspaceError> viewsOrFailed);

abstract class IApp {
  Future<Either<List<View>, WorkspaceError>> getViews({required String appId});

  Future<Either<View, WorkspaceError>> createView(
      {required String appId,
      required String name,
      String? desc,
      required ViewType viewType});

  void startWatching(
      {AppAddViewCallback? addViewCallback,
      AppUpdatedCallback? updatedCallback});

  Future<void> stopWatching();
}
