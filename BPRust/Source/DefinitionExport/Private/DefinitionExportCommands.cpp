// Copyright Epic Games, Inc. All Rights Reserved.

#include "DefinitionExportCommands.h"

#define LOCTEXT_NAMESPACE "FDefinitionExportModule"

void FDefinitionExportCommands::RegisterCommands()
{
	UI_COMMAND(PluginAction, "BPRust.DefExport", "Execute DefinitionExport action", EUserInterfaceActionType::Button, FInputChord());
}

#undef LOCTEXT_NAMESPACE
