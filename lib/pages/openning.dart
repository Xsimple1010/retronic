import 'package:flutter/material.dart';
import 'package:retronic/components/base/retro_elevated_button.dart';
import 'package:retronic/components/base/retro_icon_button.dart';

class OpeningPage extends StatefulWidget {
  const OpeningPage({super.key});

  @override
  State<OpeningPage> createState() => _OpeningPageState();
}

enum SubPages { welcome, foldSelect, binarySelect, finalize }

class _OpeningPageState extends State<OpeningPage> {
  SubPages subPages = SubPages.welcome;
  String selectedFolder = "/home/aderval/Documentos";

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: LayoutBuilder(
        builder: (context, constraints) => Stack(
          alignment: Alignment.bottomCenter,
          fit: StackFit.expand,
          children: [
            Positioned(
              bottom: constraints.maxHeight * .5,
              child: switch (subPages) {
                SubPages.welcome => Welcome(constraints: constraints),
                SubPages.foldSelect => ResourceFoldSelect(
                  constraints: constraints,
                ),
                SubPages.binarySelect => TinicBinarySelect(
                  constraints: constraints,
                ),
                SubPages.finalize => throw UnimplementedError(),
              },
            ),

            FolderSelect(
              constraints: constraints,
              subPages: subPages,
              selectedFolder: selectedFolder,
            ),

            Positioned(
              bottom: constraints.maxHeight * .1,
              child: RetroIconButton(
                onPressed: () {
                  setState(() {
                    switch (subPages) {
                      case SubPages.welcome:
                        subPages = SubPages.foldSelect;
                      case SubPages.foldSelect:
                        subPages = SubPages.binarySelect;
                      case SubPages.binarySelect:
                        subPages = SubPages.welcome;
                      case SubPages.finalize:
                        subPages = SubPages.foldSelect;
                    }
                  });
                },
                style: IconButton.styleFrom(
                  fixedSize: Size(
                    constraints.maxHeight * .08,
                    constraints.maxHeight * .08,
                  ),
                ),
                icon: LayoutBuilder(
                  builder: (context, constraints) => Icon(
                    Icons.arrow_forward,
                    size: constraints.maxHeight * .9,
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class FolderSelect extends StatefulWidget {
  const FolderSelect({
    super.key,
    required this.constraints,
    required this.subPages,
    required this.selectedFolder,
  });

  final SubPages subPages;
  final BoxConstraints constraints;
  final String selectedFolder;

  @override
  State<FolderSelect> createState() => _FolderSelectState();
}

class _FolderSelectState extends State<FolderSelect> {
  String defaultFolder = "/home/aderval/Documentos";

  @override
  Widget build(BuildContext context) {
    return Visibility(
      visible: widget.subPages == SubPages.foldSelect,
      child: Positioned(
        bottom: widget.constraints.maxHeight * .25,
        child: Column(
          children: [
            Row(
              children: [
                Text("Por padrão a pasta será cria em: "),
                Text(
                  widget.selectedFolder,
                  style: TextStyle(fontWeight: FontWeight.bold),
                ),
              ],
            ),
            Padding(
              padding: EdgeInsetsGeometry.only(
                bottom: widget.constraints.maxHeight * .03,
              ),
            ),
            RetroElevatedButton(
              onPressed: () {},
              child: Padding(
                padding: EdgeInsetsGeometry.all(5),
                child: Row(
                  children: [
                    Icon(
                      Icons.folder,
                      size: widget.constraints.maxHeight * .035,
                    ),
                    Padding(padding: EdgeInsetsGeometry.only(right: 12)),
                    Visibility(
                      visible: widget.selectedFolder == defaultFolder,
                      replacement: Text(widget.selectedFolder),
                      child: Text("Selecionar uma nova"),
                    ),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class TinicBinarySelect extends StatelessWidget {
  const TinicBinarySelect({super.key, required this.constraints});

  final BoxConstraints constraints;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Text(
          "ONDE ESTÁ O BINÁRIO DO TINIC?",
          style: TextStyle(
            fontSize: constraints.maxHeight * .06,
            fontWeight: FontWeight.bold,
          ),
        ),
        Text(
          "O BINÁRIO DO TINIC É ESSENCIAL PARA EXECUTAR AS ROMS!",
          style: TextStyle(
            fontSize: constraints.maxHeight * .022,
            fontWeight: FontWeight.w300,
          ),
        ),
      ],
    );
  }
}

class ResourceFoldSelect extends StatelessWidget {
  const ResourceFoldSelect({super.key, required this.constraints});

  final BoxConstraints constraints;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Text(
          "ONDE VAI COLOCA A PASTA DO RETRONIC ?",
          style: TextStyle(
            fontSize: constraints.maxHeight * .06,
            fontWeight: FontWeight.bold,
          ),
        ),
        Text(
          "PRINT, SAVESTATES E CONFIGURAÇÕES SERÃO SALVOS AQUI",
          style: TextStyle(
            fontSize: constraints.maxHeight * .022,
            fontWeight: FontWeight.w300,
          ),
        ),
      ],
    );
  }
}

class Welcome extends StatelessWidget {
  const Welcome({super.key, required this.constraints});

  final BoxConstraints constraints;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Text(
          "VAMOS CONFIGURAR O SEU RETRONIC",
          style: TextStyle(
            fontSize: constraints.maxHeight * .06,
            fontWeight: FontWeight.bold,
          ),
        ),
        Text(
          "A SUA CENTRAL DE EMULAÇÃO NOSTALGIA",
          style: TextStyle(
            fontSize: constraints.maxHeight * .022,
            fontWeight: FontWeight.w300,
          ),
        ),
      ],
    );
  }
}
