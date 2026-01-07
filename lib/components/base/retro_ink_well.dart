import 'package:flutter/material.dart';
import 'package:retronic/tools/game_pad_input_handle.dart';

class RetroInkWell extends StatefulWidget {
  const RetroInkWell({
    super.key,
    required this.child,
    this.onHover,
    required this.onTap,
    this.onLongPress,
    this.onFocusChange,
    this.borderRadius,
    this.focusColor,
    this.hoverColor,
  });

  final Widget child;

  final void Function(bool)? onHover;
  final Function() onTap;
  final void Function()? onLongPress;
  final Function(bool)? onFocusChange;
  final BorderRadius? borderRadius;
  final Color? focusColor;
  final Color? hoverColor;

  @override
  State<RetroInkWell> createState() => _RetroInkWellState();
}

class _RetroInkWellState extends State<RetroInkWell> {
  final GamePadInputObserver inputObserver = GamePadInputObserver();

  @override
  void initState() {
    inputObserver.start(widget.onTap, context);
    super.initState();
  }

  @override
  void dispose() {
    inputObserver.stop();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return InkWell(
      autofocus: true,
      focusNode: inputObserver.focusNode,
      onFocusChange: (value) => {
        if (widget.onFocusChange != null) {widget.onFocusChange!(value)},
      },
      onHover: (value) {
        if (widget.onHover != null) {
          setState(() {
            inputObserver.focusNode.requestFocus();
          });
          widget.onHover!(value);
        }
      },
      onTap: () {
        setState(() {
          inputObserver.focusNode.requestFocus();
        });
        widget.onTap();
      },
      borderRadius: widget.borderRadius,
      focusColor: widget.focusColor ?? Colors.transparent,
      hoverColor: widget.hoverColor,
      highlightColor: Colors.transparent,
      splashColor: Colors.transparent,
      child: widget.child,
    );
  }
}
