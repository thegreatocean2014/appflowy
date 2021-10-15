import 'dart:math';
import 'package:flowy_infra/size.dart';
import 'package:flowy_infra/theme.dart';
import 'package:flowy_infra_ui/widget/mouse_hover_builder.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:styled_widget/styled_widget.dart';

class StyledScrollbar extends StatefulWidget {
  final double? size;
  final Axis axis;
  final ScrollController controller;
  final Function(double)? onDrag;
  final bool showTrack;
  final Color? handleColor;
  final Color? trackColor;

  // ignore: todo
  // TODO: Remove contentHeight if we can fix this issue
  // https://stackoverflow.com/questions/60855712/flutter-how-to-force-scrollcontroller-to-recalculate-position-maxextents
  final double? contentSize;

  const StyledScrollbar(
      {Key? key,
      this.size,
      required this.axis,
      required this.controller,
      this.onDrag,
      this.contentSize,
      this.showTrack = false,
      this.handleColor,
      this.trackColor})
      : super(key: key);

  @override
  ScrollbarState createState() => ScrollbarState();
}

class ScrollbarState extends State<StyledScrollbar> {
  double _viewExtent = 100;

  @override
  void initState() {
    widget.controller.addListener(() => setState(() {}));
    super.initState();
  }

  @override
  void dispose() {
    super.dispose();
  }

  @override
  void didUpdateWidget(StyledScrollbar oldWidget) {
    if (oldWidget.contentSize != widget.contentSize) setState(() {});
    super.didUpdateWidget(oldWidget);
  }

  @override
  Widget build(BuildContext context) {
    final theme = context.watch<AppTheme>();
    return LayoutBuilder(
      builder: (_, BoxConstraints constraints) {
        double maxExtent;
        final contentSize = widget.contentSize;
        switch (widget.axis) {
          case Axis.vertical:
            // Use supplied contentSize if we have it, otherwise just fallback to maxScrollExtents
            if (contentSize != null && contentSize > 0) {
              maxExtent = contentSize - constraints.maxHeight;
            } else {
              maxExtent = widget.controller.position.maxScrollExtent;
            }

            _viewExtent = constraints.maxHeight;

            break;
          case Axis.horizontal:
            // Use supplied contentSize if we have it, otherwise just fallback to maxScrollExtents

            if (contentSize != null && contentSize > 0) {
              maxExtent = contentSize - constraints.maxWidth;
            } else {
              maxExtent = widget.controller.position.maxScrollExtent;
            }
            _viewExtent = constraints.maxWidth;

            break;
        }

        final contentExtent = maxExtent + _viewExtent;
        // Calculate the alignment for the handle, this is a value between 0 and 1,
        // it automatically takes the handle size into acct
        // ignore: omit_local_variable_types
        double handleAlignment = maxExtent == 0 ? 0 : widget.controller.offset / maxExtent;

        // Convert handle alignment from [0, 1] to [-1, 1]
        handleAlignment *= 2.0;
        handleAlignment -= 1.0;

        // Calculate handleSize by comparing the total content size to our viewport
        var handleExtent = _viewExtent;
        if (contentExtent > _viewExtent) {
          //Make sure handle is never small than the minSize
          handleExtent = max(60, _viewExtent * _viewExtent / contentExtent);
        }
        // Hide the handle if content is < the viewExtent
        var showHandle = contentExtent > _viewExtent && contentExtent > 0;
        // Handle color
        var handleColor = widget.handleColor ?? (theme.isDark ? theme.bg2.withOpacity(.2) : theme.bg2);
        // Track color
        var trackColor = widget.trackColor ?? (theme.isDark ? theme.bg2.withOpacity(.1) : theme.bg2.withOpacity(.3));

        //Layout the stack, it just contains a child, and
        return Stack(children: <Widget>[
          /// TRACK, thin strip, aligned along the end of the parent
          if (widget.showTrack)
            Align(
              alignment: const Alignment(1, 1),
              child: Container(
                color: trackColor,
                width: widget.axis == Axis.vertical ? widget.size : double.infinity,
                height: widget.axis == Axis.horizontal ? widget.size : double.infinity,
              ),
            ),

          /// HANDLE - Clickable shape that changes scrollController when dragged
          Align(
            // Use calculated alignment to position handle from -1 to 1, let Alignment do the rest of the work
            alignment: Alignment(
              widget.axis == Axis.vertical ? 1 : handleAlignment,
              widget.axis == Axis.horizontal ? 1 : handleAlignment,
            ),
            child: GestureDetector(
              onVerticalDragUpdate: _handleVerticalDrag,
              onHorizontalDragUpdate: _handleHorizontalDrag,
              // HANDLE SHAPE
              child: MouseHoverBuilder(
                builder: (_, isHovered) => Container(
                  width: widget.axis == Axis.vertical ? widget.size : handleExtent,
                  height: widget.axis == Axis.horizontal ? widget.size : handleExtent,
                  decoration: BoxDecoration(
                      color: handleColor.withOpacity(isHovered ? 1 : .85), borderRadius: Corners.s3Border),
                ),
              ),
            ),
          )
        ]).opacity(showHandle ? 1.0 : 0.0, animate: false);
      },
    );
  }

  void _handleHorizontalDrag(DragUpdateDetails details) {
    var pos = widget.controller.offset;
    var pxRatio = (widget.controller.position.maxScrollExtent + _viewExtent) / _viewExtent;
    widget.controller.jumpTo((pos + details.delta.dx * pxRatio).clamp(0.0, widget.controller.position.maxScrollExtent));
    widget.onDrag?.call(details.delta.dx);
  }

  void _handleVerticalDrag(DragUpdateDetails details) {
    var pos = widget.controller.offset;
    var pxRatio = (widget.controller.position.maxScrollExtent + _viewExtent) / _viewExtent;
    widget.controller.jumpTo((pos + details.delta.dy * pxRatio).clamp(0.0, widget.controller.position.maxScrollExtent));
    widget.onDrag?.call(details.delta.dy);
  }
}

class ScrollbarListStack extends StatelessWidget {
  final double barSize;
  final Axis axis;
  final Widget child;
  final ScrollController controller;
  final double? contentSize;
  final EdgeInsets? scrollbarPadding;
  final Color? handleColor;
  final Color? trackColor;

  const ScrollbarListStack(
      {Key? key,
      required this.barSize,
      required this.axis,
      required this.child,
      required this.controller,
      this.contentSize,
      this.scrollbarPadding,
      this.handleColor,
      this.trackColor})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: <Widget>[
        /// LIST
        /// Wrap with a bit of padding on the right
        child.padding(
          right: axis == Axis.vertical ? barSize + Insets.sm : 0,
          bottom: axis == Axis.horizontal ? barSize + Insets.sm : 0,
        ),

        /// SCROLLBAR
        Padding(
          padding: scrollbarPadding ?? EdgeInsets.zero,
          child: StyledScrollbar(
            size: barSize,
            axis: axis,
            controller: controller,
            contentSize: contentSize,
            trackColor: trackColor,
            handleColor: handleColor,
          ),
        ),
      ],
    );
  }
}
