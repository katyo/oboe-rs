#include "oboe/OboeExt.h"

namespace oboe {
  void AudioStreamBuilder_Initialize(AudioStreamBuilder *builder) {
    new (builder) AudioStreamBuilder();
  }
}
