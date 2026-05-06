using System;
using System.Collections.Immutable;
using Dalamud.Configuration;

namespace RemotePartyFinder;

[Serializable]
public class Configuration : IPluginConfiguration
{
    public int Version { get; set; } = 2;
    internal static readonly int CurrentVersion = 2;
    public bool AdvancedSettingsEnabled = false;
    public ImmutableList<UploadUrl> UploadUrls = DefaultUploadUrls();

    public void Migrate()
    {
        if (this.Version == 1)
        {
            // Remove findingway.io
            this.UploadUrls = UploadUrls.RemoveAll(uploadUrl => uploadUrl.Url.Equals("https://findingway.io/receiver", StringComparison.OrdinalIgnoreCase));
            this.Version = 2;
            this.Save();
        }
    }

    public static ImmutableList<UploadUrl> DefaultUploadUrls() => [
        new("https://xivpf.com/contribute/multiple") { IsDefault = true }
    ];

    public void Save()
    {
        Plugin.PluginInterface.SavePluginConfig(this);
    }
}
