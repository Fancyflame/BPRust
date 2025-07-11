// Copyright Epic Games, Inc. All Rights Reserved.

#pragma once

#include "Framework/Commands/Commands.h"
#include "DefinitionExportStyle.h"

class FDefinitionExportCommands : public TCommands<FDefinitionExportCommands>
{
public:

	FDefinitionExportCommands()
		: TCommands<FDefinitionExportCommands>(TEXT("BPRust.DefExport"), NSLOCTEXT("Contexts", "DefinitionExport", "DefinitionExport Plugin"), NAME_None, FDefinitionExportStyle::GetStyleSetName())
	{
	}

	// TCommands<> interface
	virtual void RegisterCommands() override;

public:
	TSharedPtr< FUICommandInfo > PluginAction;
};
