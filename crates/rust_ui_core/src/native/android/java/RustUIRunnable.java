// SPDX-FileCopyrightText: 2026  The rust-ui maintainers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// import android.text.TextWatcher;
// import android.text.Editable;
import java.lang.Runnable;
// import android.content.DialogInterface.OnDismissListener;
// import android.content.DialogInterface;

public class RustUIRunnable implements Runnable {
    private long rawFunctionBox;
    /// consume the rawFunctionBox making future onClick calls panic
    public native void destroy();
    /// the implementor
    @Override
    public native void run();
}
