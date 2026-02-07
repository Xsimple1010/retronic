import 'package:flutter/material.dart';
import 'package:retronic/components/game_list.dart';
import 'package:retronic/components/home_header.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  final PageController _pageController = PageController(viewportFraction: 0.66);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Color.fromRGBO(48, 47, 47, 1),
      body: LayoutBuilder(
        builder: (context, constraints) => Stack(
          children: [
            SizedBox(
              height: constraints.maxHeight,
              child: PageView(
                scrollDirection: Axis.vertical,
                controller: _pageController,
                physics: const PageScrollPhysics(
                  parent: BouncingScrollPhysics(),
                ),
                allowImplicitScrolling: true,
                children: [
                  GameList(constraints: constraints, title: "Recentes"),
                  GameList(
                    constraints: constraints,
                    title: "Nintendo - Wii",
                    label: "20",
                  ),
                  GameList(
                    constraints: constraints,
                    title: "Sony - Playstation",
                    label: "50",
                  ),
                ],
              ),
            ),

            HomeHeader(),
          ],
        ),
      ),
    );
  }
}
