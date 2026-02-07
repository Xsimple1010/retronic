import 'package:flutter/material.dart';
import 'package:retronic/components/game_item.dart';

class GameList extends StatefulWidget {
  const GameList({
    super.key,
    required this.constraints,
    this.title,
    this.label,
  });

  final BoxConstraints constraints;
  final String? title;
  final String? label;

  @override
  State<GameList> createState() => _GameListState();
}

class _GameListState extends State<GameList> {
  bool hasFocus = false;

  @override
  Widget build(BuildContext context) {
    final textColor = hasFocus
        ? Colors.white
        : Color.fromRGBO(225, 225, 225, 0.25);
    return Column(
      // crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Padding(
          padding: EdgeInsetsGeometry.symmetric(
            horizontal: widget.constraints.maxHeight * .058,
          ),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Visibility(
                visible: widget.title != null,
                child: Text(
                  widget.title ?? "",
                  style: TextStyle(
                    fontSize: widget.constraints.maxHeight * .058,
                    fontWeight: FontWeight.bold,
                    color: textColor,
                  ),
                ),
              ),
              Visibility(
                visible: widget.label != null,
                child: Text(
                  widget.label ?? "",
                  style: TextStyle(
                    fontSize: widget.constraints.maxHeight * .058,
                    fontWeight: FontWeight.bold,
                    color: textColor,
                  ),
                ),
              ),
            ],
          ),
        ),

        Container(
          height: widget.constraints.maxHeight * .51,
          width: widget.constraints.maxWidth,
          // color: Colors.blue,
          padding: EdgeInsets.only(top: widget.constraints.maxHeight * .023),
          child: ListView.builder(
            itemCount: 8,
            scrollDirection: Axis.horizontal,
            padding: EdgeInsets.symmetric(
              horizontal: widget.constraints.maxHeight * .058,
            ),
            itemBuilder: (context, index) => GameItem(
              constraints: widget.constraints,
              onFocusChange: (v) => setState(() {
                hasFocus = v;
              }),
            ),
          ),
        ),
      ],
    );
  }
}
