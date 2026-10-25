# Records each narration line of <Deck>\index.html to <Deck>\audio\<slide>-<n>.mp3,
# then prints the `files` map for publishing the deck as an artifact.
param(
  [Parameter(Mandatory)][string]$Deck,
  [string]$Voice = 'en-GB-RyanNeural',
  [string]$Rate = '+0%'
)
$Deck = (Resolve-Path $Deck).Path
$html = Get-Content -Raw -Encoding utf8 "$Deck\index.html"
$json = [regex]::Match($html, '(?s)<script type="application/json" id="narration">(.*?)</script>').Groups[1].Value
$narration = $json | ConvertFrom-Json

$audio = "$Deck\audio"
if (Test-Path $audio) { Remove-Item "$audio\*.mp3" }
New-Item -ItemType Directory -Force $audio | Out-Null
$txt = New-TemporaryFile
$files = [ordered]@{}

foreach ($slide in $narration.PSObject.Properties) {
  $id = $slide.Name
  $section = [regex]::Match($html, "(?s)<section[^>]*data-id=`"$id`".*?</section>").Value
  if (-not $section) { Write-Warning "$id has narration but no slide"; continue }
  $steps = [regex]::Matches($section, 'data-step="(\d+)"') | ForEach-Object { [int]$_.Groups[1].Value }
  $top = ($steps | Measure-Object -Maximum).Maximum
  if ($top -ge $slide.Value.Count) { Write-Warning "$id reveals step $top but has only $($slide.Value.Count) lines" }

  for ($n = 0; $n -lt $slide.Value.Count; $n++) {
    Set-Content -Path $txt -Value $slide.Value[$n] -Encoding utf8NoBOM
    $mp3 = "$audio\$id-$n.mp3"
    uvx edge-tts --voice $Voice --rate=$Rate --file $txt --write-media $mp3 2>$null
    if (-not (Test-Path $mp3)) { Write-Warning "no audio for $id-$n" ; continue }
    $files["audio/$id-$n.mp3"] = "audio/$id-$n.mp3"
  }
}
Remove-Item $txt
$files | ConvertTo-Json -Compress
