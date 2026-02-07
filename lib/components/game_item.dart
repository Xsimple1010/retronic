import 'package:flutter/material.dart';
import 'package:flutter_animate/flutter_animate.dart';
import 'package:retronic/src/bindings/bindings.dart';

final IMG_URL =
    "https://raw.githubusercontent.com/libretro/libretro-thumbnails/master/Nintendo%20-%20Wii/Named_Boxarts/Mario%20Party%208%20(USA).png";
final IMG_URL2 =
    "https://raw.githubusercontent.com/libretro/libretro-thumbnails/master/Nintendo%20-%20Super%20Nintendo%20Entertainment%20System/Named_Boxarts/Aladdin%20(USA).png";

class GameItem extends StatefulWidget {
  const GameItem({
    super.key,
    required this.constraints,
    required this.onFocusChange,
  });

  final BoxConstraints constraints;
  final void Function(bool hasFocus) onFocusChange;

  @override
  State<GameItem> createState() => _GameItemState();
}

class _GameItemState extends State<GameItem> {
  bool hasFocus = false;

  @override
  Widget build(BuildContext context) {
    final borderRadius = BorderRadius.circular(
      widget.constraints.maxHeight * .015,
    );
    return Stack(
      alignment: Alignment.center,
      children: [
        Container(
              width: widget.constraints.maxHeight * .356,
              margin: EdgeInsets.all(4),
              // height: constraints.maxHeight - 8,
              decoration: BoxDecoration(
                borderRadius: borderRadius,
                border: BoxBorder.all(
                  color: Color.fromRGBO(85, 115, 229, 1),
                  strokeAlign: 1,
                  width: 4,
                  style: BorderStyle.solid,
                ),
                // color: Colors.white,
              ),
            )
            .animate(target: hasFocus ? 1 : 0)
            .fadeIn(begin: 0, duration: 200.ms, delay: 0.ms)
            .scale(
              begin: Offset(1.02, 1.02),
              end: Offset(1.0, 1.0),
              delay: 0.ms,
              duration: 100.ms,
            ),
        Positioned(
          top: 8,
          left: 8,
          right: 8,
          bottom: 8,
          child:
              Material(
                    color: Colors.transparent,
                    child: InkWell(
                      borderRadius: borderRadius,
                      autofocus: true,
                      onFocusChange: (v) {
                        setState(() {
                          hasFocus = v;
                        });

                        widget.onFocusChange(v);
                      },
                      onTap: () {
                        LoadGameSignal(
                          corePath:
                              "/home/aderval/Downloads/RetroArch_cores(1)/RetroArch-Linux-x86_64/RetroArch-Linux-x86_64.AppImage.home/.config/retroarch/cores/snes9x_libretro.so",
                          romPath:
                              "/home/aderval/Downloads/roms/Aladdin (USA).sfc",
                        ).sendSignalToRust();
                      },
                      onHover: (v) {
                        setState(() {
                          hasFocus = v;
                        });
                      },
                      child: Ink(
                        decoration: BoxDecoration(
                          color: Colors.blue,
                          image: DecorationImage(
                            image: NetworkImage(IMG_URL),
                            fit: BoxFit.fitWidth,
                          ),
                          borderRadius: BorderRadius.circular(
                            widget.constraints.maxHeight * .015,
                          ),
                        ),
                      ),
                    ),
                  )
                  .animate(target: hasFocus ? 1 : 0)
                  .scale(
                    duration: Duration(milliseconds: 260),
                    begin: Offset(0.87, 0.87),
                    end: Offset(1, 1),
                  ),
        ),
      ],
    );
  }
}
