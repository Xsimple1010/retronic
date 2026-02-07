import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class HomeHeader extends StatelessWidget {
  const HomeHeader({super.key});

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        return Padding(
          padding: EdgeInsetsGeometry.only(
            top: constraints.maxHeight * 0.03,
            right: constraints.maxHeight * .058,
          ),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              ProgressWidget(constraints: constraints),

              Padding(padding: EdgeInsetsGeometry.only(left: 12)),
              SettingBt(constraints: constraints),
            ],
          ),
        );
      },
    );
  }
}

class ProgressWidget extends ConsumerWidget {
  const ProgressWidget({super.key, required this.constraints});

  final BoxConstraints constraints;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final fullWidth = constraints.maxHeight * 0.22;

    final progress = 0.0;

    return GestureDetector(
      onTap: () {},
      child: Container(
        width: fullWidth,
        height: constraints.maxHeight * 0.07,
        alignment: Alignment.centerLeft,
        clipBehavior: Clip.hardEdge,
        decoration: BoxDecoration(
          color: Color.fromRGBO(112, 112, 112, 1),
          borderRadius: BorderRadius.circular(constraints.maxHeight * 0.02),
        ),
        child: Stack(
          children: [
            AnimatedContainer(
              duration: Duration(milliseconds: 300),
              width: fullWidth * progress,
              height: constraints.maxHeight * 0.07,
              decoration: BoxDecoration(
                color: Color.fromRGBO(85, 115, 229, 1),
                borderRadius: BorderRadius.circular(
                  constraints.maxHeight * 0.02,
                ),
              ),
            ),
            Center(
              child: Row(
                mainAxisAlignment: MainAxisAlignment.center,
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  Icon(Icons.arrow_downward_outlined),
                  Padding(
                    padding: EdgeInsetsGeometry.only(
                      right: constraints.maxHeight * 0.008,
                    ),
                  ),
                  Text(
                    "Processando",
                    style: TextStyle(
                      fontSize: constraints.maxHeight * .02,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class SettingBt extends StatelessWidget {
  const SettingBt({super.key, required this.constraints});

  final BoxConstraints constraints;

  @override
  Widget build(BuildContext context) {
    return Container(
      width: constraints.maxHeight * 0.07,
      height: constraints.maxHeight * 0.07,
      alignment: Alignment.center,
      decoration: BoxDecoration(
        color: Color.fromRGBO(112, 112, 112, 1),
        borderRadius: BorderRadius.circular(constraints.maxHeight * 0.02),
      ),
      child: Icon(Icons.settings_outlined, size: constraints.maxHeight * .04),
    );
  }
}
