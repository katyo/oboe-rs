#include "oboe/OboeExt.h"

namespace oboe {
  void AudioStreamBuilder_new(AudioStreamBuilder *builder) {
    new (builder) AudioStreamBuilder();
  }
}
