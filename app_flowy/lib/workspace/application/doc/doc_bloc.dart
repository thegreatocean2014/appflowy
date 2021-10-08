import 'dart:typed_data';

import 'package:flowy_sdk/protobuf/flowy-workspace/errors.pb.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:app_flowy/workspace/domain/i_doc.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:flowy_editor/flowy_editor.dart';
import 'dart:convert';
part 'doc_bloc.freezed.dart';

class DocBloc extends Bloc<DocEvent, DocState> {
  final IDoc iDocImpl;

  DocBloc({
    required this.iDocImpl,
  }) : super(const DocState.loading());

  @override
  Stream<DocState> mapEventToState(DocEvent event) async* {
    yield* event.map(
      loadDoc: (_) async* {
        yield* _readDoc();
      },
    );
  }

  Stream<DocState> _readDoc() async* {
    final docOrFail = await iDocImpl.readDoc();
    yield docOrFail.fold(
      (doc) {
        final flowyDoc = FlowyDoc(doc: doc, iDocImpl: iDocImpl);
        return DocState.loadDoc(flowyDoc);
      },
      (error) {
        return DocState.loadFail(error);
      },
    );
  }

  Document _decodeListToDocument(Uint8List data) {
    final json = jsonDecode(utf8.decode(data));
    final document = Document.fromJson(json);
    return document;
  }

  Document _decodeJsonToDocument(String data) {
    final json = jsonDecode(data);
    final document = Document.fromJson(json);
    return document;
  }
}

@freezed
class DocEvent with _$DocEvent {
  const factory DocEvent.loadDoc() = LoadDoc;
}

@freezed
class DocState with _$DocState {
  const factory DocState.loading() = Loading;
  const factory DocState.loadDoc(FlowyDoc doc) = LoadedDoc;
  const factory DocState.loadFail(WorkspaceError error) = LoadFail;
}
