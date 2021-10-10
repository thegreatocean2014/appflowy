import 'package:app_flowy/workspace/presentation/app/app_page.dart';
import 'package:flowy_log/flowy_log.dart';
import 'package:flowy_sdk/protobuf/flowy-workspace/view_create.pb.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:provider/provider.dart';
import 'package:app_flowy/startup/startup.dart';
import 'package:app_flowy/workspace/domain/page_stack/page_stack.dart';
import 'package:app_flowy/workspace/presentation/view/view_page.dart';

class ViewListNotifier with ChangeNotifier {
  List<View> innerViews;
  View? _selectedView;
  ViewListNotifier(this.innerViews);

  set views(List<View> views) => innerViews = views;
  List<View> get views => innerViews;

  void setSelectedView(View view) {
    _selectedView = view;
    notifyListeners();
  }

  View? get selectedView => _selectedView;

  void update(ViewListData notifier) {
    innerViews = notifier.views;
    notifyListeners();
  }
}

class ViewListPage extends StatelessWidget {
  final List<View> views;
  const ViewListPage(this.views, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    // The ViewListNotifier will be updated after ViewListData changed passed by parent widget
    return ChangeNotifierProxyProvider<ViewListData, ViewListNotifier>(
      create: (_) => ViewListNotifier(
        Provider.of<ViewListData>(
          context,
          listen: false,
        ).views,
      ),
      update: (_, notifier, controller) => controller!..update(notifier),
      child: Consumer(builder: (context, ViewListNotifier notifier, child) {
        return _renderViews(context, notifier.views);
      }),
    );
  }

  Widget _renderViews(BuildContext context, List<View> views) {
    var viewWidgets = views.map((view) {
      final viewCtx = ViewWidgetContext(view);

      final viewWidget = ViewPage(
        viewCtx: viewCtx,
        isSelected: _isViewSelected(context, view.id),
        onOpen: (view) {
          Log.debug("Open view: $view");
          context.read<ViewListNotifier>().setSelectedView(view);
          final stackView = stackViewFromView(viewCtx.view);
          getIt<HomePageStack>().setStackView(stackView);
        },
      );

      return Padding(
        padding: const EdgeInsets.symmetric(vertical: 4),
        child: viewWidget,
      );
    }).toList(growable: false);

    return Column(
      children: viewWidgets,
    );
  }

  bool _isViewSelected(BuildContext context, String viewId) {
    final view = context.read<ViewListNotifier>().selectedView;
    if (view != null) {
      return view.id == viewId;
    } else {
      return false;
    }
  }
}
